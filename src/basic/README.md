# Basic Challenges

After finishing all the below exercises, please follow these steps below to submit to the repository

- Step 1: `Commit` your code and push to the forked Github repository

```
git add .
git commit -m "Finish Basic Challenge"
```

- Step 2: Create a `Pull Request` to merge your changes to this repository and name your PR as `Your name | Basic Challenge`

<img width="1166" alt="Screenshot 2024-04-19 at 16 23 45" src="https://github.com/openguild-labs/open-hack-rust-starter/assets/56880684/7554ca7d-da68-4a23-893a-4f2c11a78d37">

---

## Exercise 1: Print and run a "Hello World" program

### Submission Guidelines

- [x] Add your solution file `github-username.rs` to `answers/1-hello-world`

For example, if my Github username is `gavinwood`, my solution file will be under the soltuion folder as `answers/1-hello-world/gavinwood.rs`

### Main content

Go to `1-hello-world.rs`, add a code to print "Hello World"

## Exercise 2: Two Sum

### Submission Guidelines

- [x] Add your solution file `github-username.rs` to `answers/2-two-sum`

For example, if my Github username is `gavinwood`, my solution file will be under the soltuion folder as `answers/2-two-sum/gavinwood.rs`

### Main content

Given an array of integer nums and an integer target, return indices of the two numbers such that they add up to the target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

Example 1:

> Input: nums = [2,7,11,15], target = 9  
> Output: [0,1]  
> Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

Example 2:

> Input: nums = [3,2,4], target = 6  
> Output: [1,2]

Example 3:

> Input: nums = [3,3], target = 6  
> Output: [0,1]

Constraints:

> 2 <= nums.length <= 104  
> -109 <= nums[i] <= 109  
> -109 <= target <= 109

- Only one valid answer exists.

Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?

Checkpoints:

- [ ] All tests pass
- [ ] Brute force solution
- [ ] Optimize solution
