package mychaincode

import (
	"encoding/json"
	"fmt"

	"github.com/hyperledger/fabric-chaincode-go/shim"
	pb "github.com/hyperledger/fabric-protos-go/peer"
)

// AssetChaincode manages assets with transfer functionality
type AssetChaincode struct{}

type Asset struct {
	ID    string `json:"id"`
	Owner string `json:"owner"`
	Value int    `json:"value"`
}

func (t *AssetChaincode) Init(stub shim.ChaincodeStubInterface) pb.Response {
	return shim.Success
}

// Invoke handles transaction routing
func (t *AssetChaincode) Invoke(stub shim.ChaincodeStubInterface) pb.Response {
	function, args := stub.GetFunctionAndParameters()

	switch function {
	case "createAsset":
		return t.createAsset(stub, args)
	case "queryAsset":
		return t.queryAsset(stub, args)
	case "transferAsset":
		return t.transferAsset(stub, args)
	default:
		return shim.Error(fmt.Sprintf("Unknown function: %s", function))
	}
}

func (t *AssetChaincode) createAsset(stub shim.ChaincodeStubInterface, args []string) pb.Response {
	if len(args) != 3 {
		return shim.Error("Expecting 3 args: ID, Owner, Value")
	}

	id, owner, value := args[0], args[1], args[2]
	if existingAsset, _ := stub.GetState(id); existingAsset != nil {
		return shim.Error("Asset " + id + " already exists")
	}

	asset := Asset{ID: id, Owner: owner, Value: parseValue(value)}
	assetJSON, _ := json.Marshal(asset)
	if err := stub.PutState(id, assetJSON); err != nil {
		return shim.Error("Failed to write asset: " + err.Error())
	}
	return shim.Success(nil)
}

func (t *AssetChaincode) queryAsset(stub shim.ChaincodeStubInterface, args []string) pb.Response {
	if len(args) != 1 {
		return shim.Error("Expecting 1 arg: ID")
	}

	id := args[0]
	assetJSON, err := stub.GetState(id)
	if err != nil || assetJSON == nil {
		return shim.Error("Asset " + id + " not found or error: " + err.Error())
	}
	return shim.Success(assetJSON)
}

func (t *AssetChaincode) transferAsset(stub shim.ChaincodeStubInterface, args []string) pb.Response {
	if len(args) != 2 {
		return shim.Error("Expecting 2 args: ID, NewOwner")
	}

	id, newOwner := args[0], args[1]

	// Get the caller's identity
	caller, err := stub.GetCreator() // Returns the certificate of the invoking client
	if err != nil {
		return shim.Error("Failed to get caller identity: " + err.Error())
	}

	// Simplified: Assume caller ID is the owner's name (in practice, parse MSP ID or cert)
	// For demo, we'll use a stubbed caller check
	assetJSON, err := stub.GetState(id)
	if err != nil || assetJSON == nil {
		return shim.Error("Asset " + id + " not found or error: " + err.Error())
	}

	var asset Asset
	if err := json.Unmarshal(assetJSON, &asset); err != nil {
		return shim.Error("Failed to unmarshal asset: " + err.Error())
	}

	// Check if caller is the current owner (simplified for demo)
	// In real scenarios, use stub.GetCreator() to extract MSP ID or cert attributes
	callerName := "Alice" // Hardcoded for simplicity; replace with real identity check
	if asset.Owner != callerName {
		return shim.Error("Only the owner (" + asset.Owner + ") can transfer this asset")
	}

	// Update ownership
	asset.Owner = newOwner
	updatedJSON, _ := json.Marshal(asset)
	if err := stub.PutState(id, updatedJSON); err != nil {
		return shim.Error("Failed to update asset: " + err.Error())
	}

	return shim.Success(nil)
}

func parseValue(value string) int {
	var v int
	fmt.Sscanf(value, "%d", &v)
	return v
}

func main() {
	err := shim.Start(new(AssetChaincode))
	if err != nil {
		fmt.Printf("Error starting chaincode: %s", err)
	}
}
