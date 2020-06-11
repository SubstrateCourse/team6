## 第三课作业  PoE 2

课程里会给出参考资料，大家一定要自己敲一遍**代码**！

注：

1. 提交源代码，运行`cargo test`的测试结果截图，前端UI的截图；
2. 测试应覆盖所有的业务逻辑，如不同的错误场景，以及一切正常的情况下，检查存储的数据是不是预期的那样。
3. 附加题不是必答的，但可以酌情加分。
4. 代码修改在本目录 substrate-node-template 和 substrate-front-end-template 的程序文件里。

第一题：编写存证模块的单元测试代码，包括：

* 创建存证的测试用例；
* 撤销存证的测试用例；
* 转移存证的测试用例；

![Image text](https://github.com/AmadeusGB/team6/blob/lesson-3/lesson3/images/p1.png)

第二题：编写存证模块的UI，包括

* 创建存证的UI
* 删除存证的UI
* 转移存证的UI

* 创建存证latex
![Image text](https://github.com/AmadeusGB/team6/blob/lesson-3/lesson3/images/p2.png)
* 删除存证latex
![Image text](https://github.com/AmadeusGB/team6/blob/lesson-3/lesson3/images/p3.png)
* 创建存证docker
![Image text](https://github.com/AmadeusGB/team6/blob/lesson-3/lesson3/images/p4.png)
* 转移存证docker
![Image text](https://github.com/AmadeusGB/team6/blob/lesson-3/lesson3/images/p5.png)


第三题（附加题）：实现购买存证的功能代码：

* 用户A为自己的某个存证记录设置价格；
* 用户B可以以一定的价格购买某个存证，当出价高于用户A设置的价格时，则以用户A设定的价格将费用从用户B转移到用户A，再将该存证进行转移。如果出价低于用户A的价格时，则不进行转移，返回错误。


* Bob创建存证latex(设置价格：100000000000000)
![Image text](https://github.com/AmadeusGB/team6/blob/lesson-3/lesson3/images/p6.png)
* Alice花费200000000000000，购买latex存证
![Image text](https://github.com/AmadeusGB/team6/blob/lesson-3/lesson3/images/p7.png)
* 查看Alice账户，发现确实少了一部分钱
![Image text](https://github.com/AmadeusGB/team6/blob/lesson-3/lesson3/images/p8.png)
* Bob想用100000000000000，购买价格为：200000000000000，结果失败了，owner没有改变
![Image text](https://github.com/AmadeusGB/team6/blob/lesson-3/lesson3/images/p9.png)


