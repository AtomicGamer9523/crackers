<p align="center">
    <img src="./crackers/logo.png" alt="crackers-logo" style="width:10%"/>
</p>
<h1 align="center"><b>Crackers</b></h1>
<p align="center">For all your munching and cracking needs ;)</p>
<div align="center">
    <a><img src="https://img.shields.io/github/license/AtomicGamer9523/crackers"></a>
    <a href="https://www.github.com/AtomicGamer9523">
        <img src="https://img.shields.io/github/followers/atomicgamer9523?label=AtomicGamer9523%20(Me)&style=social"/>
    </a>
</div>

<h3>Examples</h3>

```bash
# Crack a sha256 hash that starts with 0xdeadbeef
crackers.exe startswith 0xdeadbeef

# Crack a sha1 hash that ends with 000 using 4 threads
crackers.exe -j 4 -t sha1 endswith 000

# Crack an md5 hash that starts with 1234 and only contains ascii characters
crackers.exe -a -t md5 startswith 1234
```
