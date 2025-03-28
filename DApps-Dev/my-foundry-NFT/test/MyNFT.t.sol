// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/MyNFT.sol";

contract MyNFTTest is Test {
    MyNFT nft;
    address owner = address(0x1);
    address user = address(0x2);

    function setUp() public {
        vm.startPrank(owner); // Start simulating calls from `owner`
        nft = new MyNFT();    // Deploy with `msg.sender` as `owner` (0x1)
        vm.stopPrank();       // Stop the prank after deployment
        console.log("Deployed MyNFT contract with owner:", owner);
    }

    function test_MintAsOwner() public {
        vm.prank(owner); // Simulate owner calling mint
        string memory uri = "ipfs://test-uri";
        nft.mint(user, uri);

        // Check ownership (tokenId starts at 1)
        assertEq(nft.ownerOf(1), user, "User should own token ID 1");
        // Check token URI
        assertEq(nft.tokenURI(1), uri, "Token URI should match");
        // Check token counter
        assertEq(nft.tokenId(), 1, "Token ID should increment to 1");
    }

    function test_MintAsNonOwnerFail() public {
        vm.prank(user); // Simulate non-owner calling mint
        string memory uri = "ipfs://test-uri";
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, user));
        nft.mint(user, uri);
    }

    function test_InitialOwner() public {
        assertEq(nft.owner(), owner, "Initial owner should be set correctly");
    }
}