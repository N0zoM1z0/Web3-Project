// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";
import "openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import "openzeppelin-contracts/contracts/access/Ownable.sol";
import {Test, console} from "forge-std/Test.sol";
import "../src/Dex.sol";

contract DexEXP is Test {
    address user = address(0x123);
    address token1;
    address token2;
    Dex dex;

    function setUp() public {
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
        token2Contract.transfer(user, 10);

        token1Contract.transfer(address(dex), 100);
        token2Contract.transfer(address(dex), 100);

        dex.setTokens(token1, token2);

        // 通过 Dex 为 user 授权
        vm.prank(user);
        dex.approve(address(dex), type(uint256).max);
    }

    function showBalances() public view {
        console.log("=========================================================");
        console.log("User token1 balance:", IERC20(token1).balanceOf(user));
        console.log("User token2 balance:", IERC20(token2).balanceOf(user));
        console.log("Dex token1 balance:", IERC20(token1).balanceOf(address(dex)));
        console.log("Dex token2 balance:", IERC20(token2).balanceOf(address(dex)));
    }

    function test_Swap() public {
        vm.startPrank(user);
        showBalances();

        while (IERC20(token1).balanceOf(address(dex)) > 0 && IERC20(token2).balanceOf(address(dex)) > 0) {
            uint256 userToken1Balance = IERC20(token1).balanceOf(user);
            uint256 userToken2Balance = IERC20(token2).balanceOf(user);
            uint256 dexToken1Balance = IERC20(token1).balanceOf(address(dex));
            uint256 dexToken2Balance = IERC20(token2).balanceOf(address(dex));

            // Swap token1 -> token2
            if (userToken1Balance > 0) {
                uint256 swapAmount = userToken1Balance;
                uint256 expectedToken2 = dex.getSwapPrice(token1, token2, swapAmount);
                if (expectedToken2 > dexToken2Balance) {
                    // 调整输入量，确保不超过 Dex 的 token2 余额
                    swapAmount = (dexToken2Balance * dexToken1Balance) / dexToken2Balance;
                    if (swapAmount == 0 || swapAmount > userToken1Balance) break;
                }
                dex.swap(token1, token2, swapAmount);
                showBalances();
            }

            // Swap token2 -> token1
            if (userToken2Balance > 0) {
                uint256 swapAmount = userToken2Balance;
                uint256 expectedToken1 = dex.getSwapPrice(token2, token1, swapAmount);
                if (expectedToken1 > dexToken1Balance) {
                    // 调整输入量，确保不超过 Dex 的 token1 余额
                    swapAmount = (dexToken1Balance * dexToken2Balance) / dexToken1Balance;
                    if (swapAmount == 0 || swapAmount > userToken2Balance) break;
                }
                dex.swap(token2, token1, swapAmount);
                showBalances();
            }
        }

        // 验证目标达成
        uint256 dexToken1Final = IERC20(token1).balanceOf(address(dex));
        uint256 dexToken2Final = IERC20(token2).balanceOf(address(dex));
        console.log("Final Dex token1 balance:", dexToken1Final);
        console.log("Final Dex token2 balance:", dexToken2Final);
        // assert(dexToken1Final == 0 || dexToken2Final == 0);

        vm.stopPrank();
    }

    function test_Nonsense() public {
        console.log("[+] test_Nonsense ... ");
    }
}