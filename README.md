# cpu_sin_curve
用CPU使用率画正弦曲线

预先计算好正弦曲线，然后按顺序遍历需要吃掉的CPU时间片，用while循环吃掉，剩下的丢给sleep，就可以用ytop进入看戏模式了
