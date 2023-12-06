pragma solidity ^0.8.1;

contract polygonInterface {
    // State variables
    string public mainnet = "Hello, World!";
    string public testnet = "Goerli";
    address public myAddress = 0x5B38Da6a701c568545dCfcB03FcB875f56beddC4;
    
    struct MyStruct {
        string mainnet;
        string testnet;
        address contractOwner;
    }
    
    // Set a state variable
    MyStruct public myStruct = MyStruct(1, "Hello, World!");
    
    // Local variables
    function getValue() public pure returns(uint) {
        uint value = 1;
        return value;
    }

    // Create connectors for Polygon Networks

        // Plasma Bridge

        // PoS Bridge
}
