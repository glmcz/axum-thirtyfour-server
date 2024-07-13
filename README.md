# axum-thirtyfour-server


As it is stated in the paper for servers with restricted num of core (less than 4) it is better to use Locks (Preferable are Spin locks)
instead of channels (msg passing between threads)[1]. But async with channel opening us a way
for better HW scalability and worse condition for debugging.

![img.png](img.png)

Reference:
[1] https://sigops.org/s/conferences/sosp/2013/papers/p33-david.pdf
