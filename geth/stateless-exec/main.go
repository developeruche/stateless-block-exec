package main

import (
	"encoding/json"
	"fmt"
	"io"
	"os"

	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core"
	"github.com/ethereum/go-ethereum/core/stateless"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/core/vm"
	"github.com/ethereum/go-ethereum/params"
	"github.com/ethereum/go-ethereum/rlp"
)

type StatelessInputRaw struct {
	Block   *types.Block      `json:"block"`
	Witness *hexutil.Bytes `json:"witness"`
}

// StatelessInput represents the input data for stateless block execution
type StatelessInput struct {
	Block   *types.Block      `json:"block"`
	Witness *stateless.Witness `json:"witness"`
}

func main() {
	fmt.Println("Starting stateless block execution")

	input := obtainInput()
	chainConfig := obtainChainConfig()
	
	fmt.Println("Input obtained")

	stateRoot, receiptRoot, err := core.ExecuteStateless(chainConfig, *obtainVmConfig(), input.Block, input.Witness)
	if err != nil {
		panic(fmt.Sprintf("Block execution failed: %v", err))
	}

	fmt.Printf("Block execution completed successfully:\n")
	fmt.Printf("- State Root: %s\n", stateRoot.Hex())
	fmt.Printf("- Receipt Root: %s\n", receiptRoot.Hex())
}

// obtainInput reads and parses the block and witness data from a JSON file
func obtainInput() *StatelessInput {
	// Path to the JSON file containing block and witness data
	path := "block_and_witness.json"

	// Open the file
	file, err := os.Open(path)
	if err != nil {
		panic(fmt.Sprintf("Could not open %s: %v", path, err))
	}
	defer file.Close()

	// Read the file content
	jsonContent, err := io.ReadAll(file)
	if err != nil {
		panic(fmt.Sprintf("Failed to read file content from %s: %v", path, err))
	}

	// Parse the JSON content into StatelessInput
	var input StatelessInputRaw
	if err := json.Unmarshal(jsonContent, &input); err != nil {
		panic(fmt.Sprintf("Failed to parse %s as StatelessInput: %v", path, err))
	}

	var inputOut StatelessInput
	inputOut.Block = input.Block
	witness := new(stateless.Witness)
	err = rlp.DecodeBytes(*input.Witness, witness)
	
	if err != nil {
		panic(fmt.Sprintf("Failed to decode witness data: %v", err))
	}
	
	fmt.Println(witness)
	
	inputOut.Witness = witness
	return &inputOut
}

// obtainChainConfig returns the fork configuration for block execution
func obtainChainConfig() *params.ChainConfig {
	// Using Prague fork configuration (similar to the Rust implementation)
	return params.MainnetChainConfig
}


// obtainVmConfig returns the VM configuration for block execution
func obtainVmConfig() *vm.Config {
	// Using default VM configuration
	return &vm.Config{}
}
