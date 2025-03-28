// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";
import "openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import "openzeppelin-contracts/contracts/access/Ownable.sol";
import {Test,console} from "forge-std/Test.sol";
import "../src/Dex.sol";

contract DexEXP is Test{
    address user = address(0x123);
    address token1;
    address token2;
    Dex dex;
    function setUp() public{
        dex = new Dex();
        SwappableToken token1Contract = new SwappableToken(
            address(dex), 
            "Token1", 
            "TKN1", 
            10000
        );
        SwappableToken token2Contract = new SwappableToken(
            address(dex), 
            "Token2", 
            "TKN2", 
            10000
        );
        token1 = address(token1Contract);
        token2 = address(token2Contract);

        token1Contract.transfer(user, 10);
        token2Contract.transfer(user, 100);

        token1Contract.transfer(address(dex), 100);
        token2Contract.transfer(address(dex), 1000);

        dex.setTokens(token1,token2);

        // authorize user to token1&&token2
        vm.prank(user);
        IERC20(token1).approve(address(dex), type(uint256).max);
        vm.prank(user);
        IERC20(token2).approve(address(dex), type(uint256).max);

    }
    
    function test_Swap() public {
        console.log("\n[+] ROUND 1");
        console.log("Initial balances:");
        console.log("User token1:", IERC20(token1).balanceOf(user));
        console.log("User token2:", IERC20(token2).balanceOf(user));
        console.log("Dex token1:", IERC20(token1).balanceOf(address(dex)));
        console.log("Dex token2:", IERC20(token2).balanceOf(address(dex)));

        // 测试交换
        vm.prank(user);
        dex.swap(token1, token2, 5); // 用5 token1换token2
        
        console.log("\nAfter swap:");
        console.log("User token1:", IERC20(token1).balanceOf(user));
        console.log("User token2:", IERC20(token2).balanceOf(user));
        console.log("Dex token1:", IERC20(token1).balanceOf(address(dex)));
        console.log("Dex token2:", IERC20(token2).balanceOf(address(dex)));

        console.log("\n[+] ROUND 2");
        console.log("Initial balances:");
        console.log("User token1:", IERC20(token1).balanceOf(user));
        console.log("User token2:", IERC20(token2).balanceOf(user));
        console.log("Dex token1:", IERC20(token1).balanceOf(address(dex)));
        console.log("Dex token2:", IERC20(token2).balanceOf(address(dex)));

        // 测试交换
        vm.prank(user);
        dex.swap(token1, token2, 5); // 用5 token1换token2
        
        console.log("\nAfter swap:");
        console.log("User token1:", IERC20(token1).balanceOf(user));
        console.log("User token2:", IERC20(token2).balanceOf(user));
        console.log("Dex token1:", IERC20(token1).balanceOf(address(dex)));
        console.log("Dex token2:", IERC20(token2).balanceOf(address(dex)));


        console.log("\n[+] ROUND 3");
        console.log("Initial balances:");
        console.log("User token1:", IERC20(token1).balanceOf(user));
        console.log("User token2:", IERC20(token2).balanceOf(user));
        console.log("Dex token1:", IERC20(token1).balanceOf(address(dex)));
        console.log("Dex token2:", IERC20(token2).balanceOf(address(dex)));

        // 测试交换
        vm.prank(user);
        dex.swap(token2, token1, 100); // 用5 token1换token2
        
        console.log("\nAfter swap:");
        console.log("User token1:", IERC20(token1).balanceOf(user));
        console.log("User token2:", IERC20(token2).balanceOf(user));
        console.log("Dex token1:", IERC20(token1).balanceOf(address(dex)));
        console.log("Dex token2:", IERC20(token2).balanceOf(address(dex)));
    }
    
    function test_TokenBalances() public view {
        // 只是查看余额，不需要修改状态
        console.log("User token1 balance:", IERC20(token1).balanceOf(user));
        console.log("User token2 balance:", IERC20(token2).balanceOf(user));
        console.log("Dex token1 balance:", IERC20(token1).balanceOf(address(dex)));
        console.log("Dex token2 balance:", IERC20(token2).balanceOf(address(dex)));
    }
    function test_Nonsense() public{
        console.log("[+] test_Nonsense ... ");
    }
}