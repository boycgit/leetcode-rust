const { generateTemplateFilesBatch } = require('generate-template-files');
const fs = require('fs');

const config = require('./package.json');
const path = require('path');
const { getInfoFromFile } = require('./tools/utils');

// 获取命令行参数
const args = process.argv;

if(args.length < 3) {
  console.log('缺少文件名参数，请重新执行');
  process.exit(1);
}

const curFileName = args[2];
console.log(`尝试从 ${curFileName}.rs 文件中获取信息`);

// 同步读取目标文件并提取信息
const sourceFileStr = fs.readFileSync(
  path.join(__dirname, 'src', `${curFileName}.rs`),
  'utf8'
);
const sourceFileInfo = getInfoFromFile(sourceFileStr);


// 以 append 模式打开需要改写的文件
const mainStream = fs.createWriteStream('./src/main.rs', { flags: 'a' });
const testStream = fs.createWriteStream('./src/tests.rs', { flags: 'a' });


// 定义一个功能函数，将新增模块注册到入口文件中
function appendToFile(filename) {
    const str = `\n#[path="./${filename}.rs"]\nmod ${filename};`;
    const testStr = `\n#[path="./tests/${filename}_test.rs"]\nmod ${filename}_test;`;
    
    // 将字符串写入流中
    mainStream.write(str + '\n');
    testStream.write(testStr + '\n');
}

generateTemplateFilesBatch([
  {
    option: '创建测试文件',
    defaultCase: '(noCase)',
    entry: {
      folderPath: './tools/templates/',
    },
    dynamicReplacers: [
      { slotValue: curFileName, slot: '__name__' },
      { slotValue: sourceFileInfo.argsWithType, slot: '__argsWithType__' },
      { slotValue: sourceFileInfo.functionName, slot: '__fnName__' },
      { slotValue: sourceFileInfo.args.map(o=>o.name).join(', '), slot: '__argsList__' },
    ],
    output: {
      path: './src/tests',
      pathAndFileNameDefaultCase: '(noCase)',
      overwrite: true,
    },
    onComplete: (results) => {
      console.log(`创建样例文件完成：`, results);
      const { stringReplacers } = results;
      // 获取用户输入的文件名，动态改写 tests.rs / main.rs 内容
      const filename = stringReplacers[0].slotValue;
      if (filename) {
        appendToFile(filename);
      }
    },
  },
]);

// 在程序退出时销毁流
process.on('exit', () => {
  mainStream.destroy();
  testStream.destroy();
  console.log('程序退出');
});
