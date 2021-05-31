/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} l1
 * @param {ListNode} l2
 * @return {ListNode}
 */

 function ListNode(val, next) {
     this.val = (val===undefined ? 0 : val)
     this.next = (next===undefined ? null : next)
 }

 var addTwoNumbersBad = function(l1, l2) {
    let res = new ListNode;
    let head = res;
    let remainder = 0;
    while(true) {
        let d1 = (l1 != null) ? l1.val : 0;
        let d2 = (l2 != null) ? l2.val : 0;
        let sum = d1 + d2 + remainder;
        if(sum <= 9) {
            remainder = 0;
            res.val = sum;

        }
        else {
            let numStr = sum.toString();
            remainder = Number(numStr.charAt(0));
            res.val = Number(numStr.charAt(1));

        }
        l1 = (l1 != null) ? l1.next : null;
        l2 = (l2 != null) ? l2.next : null;
        if(l1 == null && l2 == null) {
            if(remainder != 0) {
                res.next = new ListNode(remainder);
            }
            break;
        }
        res.next = new ListNode();
        res = res.next;
    }
    return head;
}

function addTwoNumbers(l1, l2) {
    let res = new ListNode;
    let head = res;
    let remainder = 0;
    while(l1 || l2) {
        let d1 = (l1 != null) ? l1.val : 0;
        let d2 = (l2 != null) ? l2.val : 0;
        let sum = d1 + d2 + remainder;
        remainder = Math.floor(sum / 10);
        res.next = new ListNode(sum % 10);
        res = res.next;
        l1 = (l1 != null) ? l1.next : null;
        l2 = (l2 != null) ? l2.next : null;
    }
    if(remainder != 0) {
        res.next = new ListNode(remainder);
    }
    head = head.next;
    return head;
};

let l1 = new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9)))))));
let l2 = new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9))));
let res = addTwoNumbers(l1, l2);

let out = [];
while(res != null) {
    out.push(res.val)
    res = res.next;
}
console.log(out)