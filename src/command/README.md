# Command パターン

>Commandパターンは命令/動作をオブジェクトで表現するオブジェクト指向プログラミングデザインパターンの一種である。
>
>リクエストのために必要な手続きとデータをCommandオブジェクトとしてカプセル化した上で取り回し、必要に応じてExecute（実行）するパターンである。オブジェクトであることを生かして命令のキューイングやロギング、Undo等が可能になり、Executeを分離したことで手続きと実行を疎結合にできる

## 定義

1. 処理をメソッドとして内包するCommandクラスの定義
2. Commandオブジェクトの生成
3. Command.Execute()のコールによるリクエスト実行

## 利点

>このパターンがもつ利点の1つはリクエスト依頼処理と実装処理の分離（疎結合）である[8]。
>
>例えばクリックでリクエストを実行できるUIフレームワークを開発するとする。ボタンのクリック機能とリクエストの実行はフレームワークの責務だが、リクエストで具体的に何が起こるかはアプリケーションの責務である。すなわちUIフレームワーク側はクリックに応じてリクエストを発行するが、リクエストに対してどのような処理がおこなわれるのか、そもそもリクエストの受け手が誰なのか知らない[9]。ここでボタンの生成時にCommandを受け入れるとする。UIフレームワーク側はCommandがどんな処理を内包しているかは（カプセル化されているので）わからないが、.Execute()を実行すればリクエストが実行されることは知っている。このinterfaceを介した契約により、クリック時にcommand.Execute()するだけでクリックに応答したリクエストを実行できる。このようにCommandパターンはCommandの実行と実装を疎結合にできる。言い換えれば、CommandパターンはCommandオブジェクトのDIによる処理と実行の分離（関心の分離）である。これは手続き型プログラミングにおけるcallbackに相当する[10]。
>
>このパターンがもつもう1つの利点はCommandが独立したオブジェクトである点である。未実行のCommandオブジェクトを配列にいれればキューイングが可能であり、それに応じた非同期処理・スケジューリングが可能になり、実行済みのCommandをキューイングすれば履歴保存とロギング・Undoなどが可能になる。

## Ref

>
> <https://refactoring.guru/design-patterns/command>
>
> <https://zenn.dev/morinokami/books/learning-patterns-1/viewer/command-pattern>
>
> <https://ja.wikipedia.org/wiki/Command_%E3%83%91%E3%82%BF%E3%83%BC%E3%83%B3>
>
> <https://keens.github.io/blog/2017/05/06/rustkazenidezainpata_n23tane/>
>
> <https://qiita.com/mopp/items/3794dc955f7dc9d8ca63#command-%E3%83%91%E3%82%BF%E3%83%BC%E3%83%B3>
>
> <https://qiita.com/skyc_lin/items/0b4bbebd19225eaed054>
