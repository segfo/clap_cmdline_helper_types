/// CmdlineResultがMsgのときに使用されます。
/// メッセージが、どのようなきっかけで生成されたかを表します。
pub enum CmdlineMsgHint {
    /// -Vオプションが指定されたことを表します。
    Version,
    /// -hまたは--helpオプションが指定されたことを表します。
    Help,
    /// 返却されるメッセージ文字列はHelpと変わりませんが、パースエラーであることを表します。
    PerseErrorHelp,
    None,
}
pub enum CmdlineResult<T> {
    /// 引数のパースが成功しています。
    Ok(T),
    /// String: ヘルプメッセージまたはバージョンメッセージなど、コンソールに表示されるメッセージを表現しています。
    /// CmdlineMsg: どのような状況でメッセージが表示されようとしているのか、ヒント情報を返します。
    Msg(String, CmdlineMsgHint),
}