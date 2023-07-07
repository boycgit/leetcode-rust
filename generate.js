const { generateTemplateFiles } = require('generate-template-files');
const fs = require('fs');

const config = require('./package.json');

// 以 append 模式打开
const mainStream = fs.createWriteStream('./src/main.rs', { flags: 'a' });
const testStream = fs.createWriteStream('./src/tests.rs', { flags: 'a' });

// 定义一个功能函数，将新增模块注册到入口文件中
function appendToFile(filename) {
    const str = `#[path="./${filename}.rs"]\nmod ${filename};`;
    const testStr = `#[path="./tests/${filename}_test.rs"]\nmod ${filename}_test;`;
    
    // 将字符串写入流中
    mainStream.write(str + '\n');
    testStream.write(testStr + '\n');
}

generateTemplateFiles([
  {
    option: '创建测试文件',
    defaultCase: '(noCase)',
    entry: {
      folderPath: './tools/templates/',
    },
    stringReplacers: [{ question: '输入文件名', slot: '__name__' }],
    output: {
      path: './src/tests',
      pathAndFileNameDefaultCase: '(noCase)',
      overwrite: true,
    },
    onComplete: (results) => {
      console.log(`创建样例文件完成：`, results);
      const {stringReplacers} = results;

      // 获取用户输入的文件名，动态改写 tests.rs / main.rs 内容 
      const filename = stringReplacers[0].slotValue;
      if(filename){
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
