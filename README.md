# Rust leetcode 刷题

本仓库记录自己采用 Rust 刷 leetcode 的过程

用了 leetcode 中文插件，所以本文下载下来的题目是中文格式的（虽然有点扯），不过仍旧是可以当成正常的 Rust 仓库运行

可以前往 src/tests 目录下查看各个题解的 test 用例 —— 默认支持 VS Code 的 debug 功能，方便调试源码和理解

## 快速生成测试文件（模板）
如果手动创建测试案例文件还是比较麻烦，需要不断地 ctrl+c/ctrl+v，这里使用 [generate-template-files](https://github.com/codeBelt/generate-template-files/tree/master) 来协助生成测试模板文件

### 根据文件名一键配置测试文件
执行以下命令：
```javascript
node scripts/generate.js [文件名]
```

会自动生成测试文件、更新入口文件信息
> 注意：生成的测试文件内容可能会有语法报错，需要自己手动更改


### 重塑入口文件
执行以下命令：
```javascript
node scripts/reshape.js
```
会根据 src 目录下已经有的测试文件重新生成 main.rs 和 test.rs 文件。
> 之所以有这个命令，是因为有可能多次执行上述的 `node scripts/generate.js` 命令导致入口文件变得混乱，使用该命令可以清理恢复