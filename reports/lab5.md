
## 功能实现
### 1.spawn
参考`initproc`的初始化过程学习对于新 task 启动时怎么初始化 TCB. 参考 `fork` 和 `exec` 的过程学习怎么启动一个新的进程。所以`spawn`的主要流程就是根据文件名读取
一个文件的`elf_data`然后初始化 tcb，加入任务队列中即可。

### 2.stride调度算法
stride 算法原理比较清晰，只需要修改 `TaskManager`里的 `add_task`和 `fetch_task`的接口内容就好了。需要注意的是我根据问答题里面的内容实现了`Stride`类用于用于处理溢出的情况。

## 问答题

- 如果没有处理好溢出的话，不一定是 p1 限制性。因为溢出之后变成了 `p1.stride = 9,p2.stride=4`，就是 p2 先执行了。

- 因为每次加的数都会小于等于 BigStride / 2，如果把0-255看成一个环，那么两个数之间的最大距离就是 BigStride/2 ，所以距离就只能是小于等于 BigStride/2了

- 
```rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let max = cmp::max(self.0, other.0);
        let min = cmp::min(self.0, other.0);
        if max == min {
            Some(cmp::Ordering::Equal)
        } else if max - min <= BIG_STRIDE / 2 {
            return self.0.partial_cmp(&other.0);
        } else {
            return other.0.partial_cmp(&self.0);
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

```
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

独立完成

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

无参考资料

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。