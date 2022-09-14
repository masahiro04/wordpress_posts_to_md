```metadata
{
    "title": "NextPage<Props>でhoge is missing in props validationが出る",
    "date": "2022-05-06T14:59:11",
    "categories": "Next.js, React",
}```

```
 React.FC では発生しないエラーがNextPageを使ったところ発生しました
```

```typescript
 
type Props = {
  nodes: Array<Node>;
};

const Index: NextPage<Props> = (props) => {
  const { nodes } = props;
  // 省略
}
```

![GitHubでリビジョン管理](./Screen-Shot-2022-05-06-at-14.46.48-644x216.png)

最初に出てきた案(ググった)としては、eslintに下記を追加して警告自体をなくす、というものです

```json
 rules: {
  'react/prop-types': 'off'
}
```

当然こちらで実装することは可能なのですが、TypeScriptを利用していて、わざわざその恩恵を排除するというのはチーム開発上選択でいないのでこちらはなしにしました

そして次に見つけたのが記事

[this is a page in nextjs and the same error appear in my components too.the problem is that eslint can’t understand the prop interface when I’m using arrow function.https://github.com/jsx-eslint/eslint-plugin-react/issues/2777#issuecomment-683555883](https://github.com/jsx-eslint/eslint-plugin-react/issues/2777#issuecomment-683555883)

通常のReact.FC等ではeslintが解析できていますが、NextjsになるとNextjsの型を認識することができず、それが問題となって、<strong><span class="has-inline-color has-luminous-vivid-amber-color">Propsを入れているけどそれをeslintが認識できていない</span></strong>、という状況に陥っていたっぽいです

なのでこのように明示的に型を追加することで対処してみたところ、eslintの警告はなくなり、綺麗に実装することができました！

![GitHubでリビジョン管理](./Screen-Shot-2022-05-06-at-14.55.39-644x197.png)
