# 实现功能
- 兼容``sys_get_time``, ``sys_mmap``和``sys_munmap``系统调用
- 实现``sys_spawn``系统调用，复用``TaskControlBlock ``的``new``方法，生成一个new_task，并将它的父进程设置为当前进程，以及将当前进程的子进程加入该new_task即可
- 实现stride调度算法，在TCB结构体中加入stride和pass字段，修改``TaskManager ``的``fetch``方法，遍历当前的队列，找到stride最小的TCB返回，对返回的TCB的stride字段累加上pass字段

# 问答题
- 实际情况是轮到 p1 执行吗？为什么？
  
  不是，u8类型整数会溢出，p2.stride会变为4，小于p1.stride

- 为什么？尝试简单说明（不要求严格证明）。
  
  因为prio大于等于2时，pass最大为BigStride / 2，只要初始时满足STRIDE_MAX – STRIDE_MIN <= BigStride / 2，STRIDE_MIN增加一次pass后不可能比STRIDE_MAX大超过BigStride / 2

```Rust
use core::cmp::Ordering;
use config::BIG_STRIDE;

struct Stride(u8);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if (self.0 > other.0 && self.0 - other.0 >= BIG_STRIDE / 2) {
            Some((self.0 + (255 as u8)).cmp(&other.0)
        } else if (self.0 < other.0 && other.0 - self.0 >= BIG_STRIDE / 2) {
            Some((other.0 + (255 as u8)).cmp(&self.0))
        } else {
            Some(self.0.cmp(&other.0))
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```

# 荣誉准则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。