
fn get_start_index(data : &str, count: usize) -> usize {

  for i in 0..data.len() - count {
    let part = &data[i..i+count];

    let mut chars: Vec<char> = part.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    
    let mut no_dupe = true;
    for idx in 0..chars.len() - 1 {
      if chars[idx] == chars[idx + 1] {
        no_dupe = false;
      }
    };

    if no_dupe {
      return i + count;
    }
  }
  unreachable!();
}

fn main() {
  let start = get_start_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 4);
  assert!(start == 5, "Wrong start seq");

  let start = get_start_index("nppdvjthqldpwncqszvftbrmjlhg", 4);
  assert!(start == 6, "Wrong start seq");

  let start = get_start_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4);
  assert!(start == 10, "Wrong start seq");

  let start = get_start_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4);
  assert!(start == 11, "Wrong start seq");

  let start = get_start_index("qnjjqgjqgglqqwrrvvcwwtjjzpjjfwwtwzwmzmrmttrvrpppbtbllhrrwtwzwjwzznfzfvvvngnrrhzrhhsmsrrsrrtqrrpzpnpfphfhllwlclfcfrrqffhvhsvswvswwngnzzcmmnrnzrnnwhwgwvggpcggnrntrrgmgdgmmgzmzssprrmddjndnsnrsrddnmdmqddrzznwznndfnndjjmgjmjmmnpmnmrmzmfzmmwhwzwrwqqqhshhncnvvhfvhfhzzvttrnrdnnwddptphpthhbgbgmbbvppnhphmmncnwcwqwppgddsqqjqppcrcncvnccjsjzztczttjzzwwhshnsnttrctchthnnswnnfzfhhsfhhdccgwcggbrblrlwlrlnntsscffpnnwffqgqdqhqhphhvtvmvhmmnssqbqfqfwwbfwbfwwzwgwgpgwwchcshslhssqsppqdqhhnssrzzvbbhjjqdjdgjgttvzzmvzvvnmvvfbbncbcqcpqpbbcjcncjnccjfjpjdjvdddbzzfrfwfpwpfwppvjvfjjhfhvvqttfhffscsvvbqbzqqjssbmsbshsrshsphhwhvwhwcwwnjjlzzfgfrgrwggfddpqpqqcmmtwtcwttgjjnfnnjvvmfmccvhchjcjmccmmwgghccnffjccqczqqbdbrdrsdsffmzzndnncbnbgbsshbhvbvsbvsbbvjbvbdvvqpqnpngpnggbbndbbdzdhzdhhjcjjmbbqgqbbfrrvzvcvffhggcbcjbbqtbbszbbvvcnvnccldddlrrtqrttpjpnjpnpznnqmnnrrmmsdmdbbffcwwllnffssspvvrjjwhwmmqhmmhjjtnjtjgttnrtnnsgsbssffstssjvjwwwwnjngjnnnlndndqdqqwdwcdcsccsgssgrrqwqqqhgqgfgffmnffzjjssqsfqqgddgcgtgrrvgrrdhdcdgdbdsswgwwpwggdfggnjnnvffzhhcqclqclcdlldbldlffdrrzttnhthdthhrthhgcgllzqlzzfhhmshssdvvtnnpngpprqrrrbnbssnhnnqdndsnshhzjjrnjnmjjwfffpbpnbbwddpwptwwpbpffvggmjmgghrrnqqhnhsnhsnnnlddmjddrdrhrchhgmgfmfdfcfpprsrzzzmhzhmhrmmrwmmpdmpmcclmlvvlffhlhhczccsvvptvpvwvdvqqdvqdddvnvsnswwspwpqqpddpvvsqsjjwnwrrjbjtttdhhhwbhbzzqrqfrrzlrzrqrbqrqzzfczfzrrqffnrnhrrcrtrvrffvflfsllcjcvvmqmlqqwjwgglhlvldltlqqdcddtssltsllsnndvdnddfqdffprrbssgccqmqrrnsrnnlzlhhqrhqhcqcjqcqpcqqmvqqgbgpbggnfggdpgpvpgvvsrrhqhshmsmwsmwssvnvtnttphpdpdspsvslshhlplmplmmljjvnnsrslrrvttqqprrptrptpnnmmflltntrrnjjcljcllwcllddbgbjbrrtwrtrftfctftsslsggggzqqhhrvvdqqwqbqtthdhrrpwpqpmmqcmqmllsmllhvfwbfvgzqgbhlfhqrqtzfpplgjtgngzrdfltnqlwsbhmwdfvrdjlgwftjvmdsgdgnswhrtmzgfqfsfnczjnmgqcfzvwlbgzsrpcbwwtrmbqtqhdmhmscqgjgpqdqcrvdvwplpdzsjqbvgpqcvfspqrcsjlhrqpjmdszjpqhmdwtddwqhbwsrlcjpzwsjjvbzcllqfwbhfvjqbqfbsrbgvgchdmgqjnvdrzlmrnlpzrljgjvtrdtqnzbnhpgpgjvwttmnfbpvrtmpstbmtwdwfzvznrwmspftgvrmdfwqltzzmlgrvvwjgdblnnbjzjfqpldsqbhrstnhrjqfcmzcrtqcpqmmfqzndgjwtcgnwrgdznzdgzbvmrlvrjvjgmfcrmrjbpjwqvhprbphnqsbpcpnflpgnpgggqpgrwghfpfvdljjqnvqgbvcpjbjlqghjppfhzfzczmcwnhrjzzrwlfqhvdwqrcbvprclnmqmqwdrhgtswwqhqtcjhndrmcrzdjnvwsstzplhcdzwzqbjjthsmjrmpfbqljlmnvmfddqflnhrfhchzgnlbcpvppnspsdtnqmfhrztznhltmqlrwzgczqmjggvnphwpvrwcnqhsdfglblvvdlqfnghtzgngtjnrzsjgwcglsrczlfqpmmzsrqwcclslzjpvpfgcwzwsfmwwswdsvnvtmwzmhwzvnnnldnjljbwlglpjvrggbmvhctsggtjgrmjrglnhtjzwhrvtmfmphgmpdmvlhzrhrtjcwswlnnjphsschrdvstmflzqsbqmqggjjdsdtjhcvjlzclbpvnbjgngnzvtdgszdtrwpnjmgwpcjpbsvrsmdjdjdbhqqvplbmfhmbgmtggvtltqjphlggscdbzncqhqqlmflmjbpnhbnqjcbfmggnftjjbzbrqwcgvfhsddrqgrvnztzzqzwjrjprmqctlttmgltdgrcvnbrbzrsbbqnsrslrswvmrqlctvsdfmcrdddcdwvfmwwcbqsdhmmgplzhnsfvhtctmvrzntvptpnbhhvjlphmtcctlhdvtvvqbmbczjqfvnqwzbgjzlgnqhvbjjnvrgfprvllltqdwvvmchrdqfczmmbmcjwtwgqbfjzgpcfzhhnwcghqltrlhlntjcrthpsbmcdzzqljdrdhfctwrzlvhdjvgmpscssmhdggrnhfbtfpqtsbqnqjfjwlbdfcpzfjtspnrjzshjcrrzwclddqqbplwghrcjhfqgblfrlrdhmdqmrzdnqthvvzlcjmqzqtgbcwcprpnjtbhsqmzlprhmssltfvvgqzzpnjtsmplhfpsnznjwrrgvmbbvjzzwctmwcwcwjwvmlpcmlmrfqbdjpclsqjnsnndzqfcghqsmhqcjwjjbbwmsttwnththhhgrlhrtrcjppvmjlqtqhpbhpsdhzcqpmqqbvwrlvjrnmlfvtntwghtphwzqmbmmpgvfrqsjwffdbcjjbwrrntrhnjwjfprzjcpnwvtwjcdhppspbdbwnzdbhvpqljbrfnpnbqdtqzdjlbvgbvvhcjrqwlqmhnhjfppllrsnsvnmttvrsdrqjfrdvhplwbhhnvglvgcvtzblfswzrrbwpjnslthjmgsdgwqrfjfplmbgsdcltzmdhrbpqggnjrcddltvdbrlhtzwfghbpnlmghwqdtpctmqbggfzflfzjgcbvfqtztsmgdvdlrrdrtmmjbtdtjvvjvlwwlwnzhlfqtrmdgvlflmbffnzlvtccnmcddrqfhgzlhngrfmsqdrnshshfhhtnwmnqmjtzggdfmpgcsrhjzhhvlppfnqmjsvtcwwjdvdrgrldbdnrqtvjjnddgmfdtqrncdlfznfpbqldztzzgpvwhvnqrrzdwsrdvcnslmwcdztvqzgbshpqrdbnmlfltpbsfrnbbcwvwvjnrdspwrwzpljqwwvqhbhhqqjnlrgwqvvsjrscbfqvhbwtjvwzmwzvftlwmtbmglvqvlnwrzvwclvzjcvgfstjpwdbtbhmnqzfgpqmthpchrrqfpqfflwhwrhmdzvgbzlmfgzvhrldprlgrvjcvjhslghqddfvqzgplmtfqjzhpctvtgrzjhwrjzbsqqrcztrstbvdjbtszrbvvzvwzfclsnrftcqtqdghdrnhglzptzwtqtgnfsqhmvnqqwtnwszgtvpnndbzcwgwhpcdlsdpptvlcqtmfhfjzbvfsvnjvqrrjqbggnjqfrrrvqmzscvlltfwzdlzqstlrrmqqmbmbtgbqjsvbncblddgzvlstmrpmrlfcqpvwthgbhlvfcfcwvslcjnztfwgdgwjfrwrzbbszmtvzchvstcfrgqllfrdccrjmtspshgqbzcldddqgjnpdsctmjphcwqvjtvmqlvvpzdgjdlpwdrjbshhrgjtglwnzlzsqngspbzgbwpgmfcv", 4);
  println!("Start is at {}", start);
  assert!(start == 1625, "Wrong start seq");

  let start = get_start_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14);
  assert!(start == 19, "Wrong start seq");

  let start = get_start_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 14);
  assert!(start == 23, "Wrong start seq");

  let start = get_start_index("nppdvjthqldpwncqszvftbrmjlhg", 14);
  assert!(start == 23, "Wrong start seq");

  let start = get_start_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14);
  assert!(start == 29, "Wrong start seq");

  let start = get_start_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14);
  assert!(start == 26, "Wrong start seq");

  let start = get_start_index("qnjjqgjqgglqqwrrvvcwwtjjzpjjfwwtwzwmzmrmttrvrpppbtbllhrrwtwzwjwzznfzfvvvngnrrhzrhhsmsrrsrrtqrrpzpnpfphfhllwlclfcfrrqffhvhsvswvswwngnzzcmmnrnzrnnwhwgwvggpcggnrntrrgmgdgmmgzmzssprrmddjndnsnrsrddnmdmqddrzznwznndfnndjjmgjmjmmnpmnmrmzmfzmmwhwzwrwqqqhshhncnvvhfvhfhzzvttrnrdnnwddptphpthhbgbgmbbvppnhphmmncnwcwqwppgddsqqjqppcrcncvnccjsjzztczttjzzwwhshnsnttrctchthnnswnnfzfhhsfhhdccgwcggbrblrlwlrlnntsscffpnnwffqgqdqhqhphhvtvmvhmmnssqbqfqfwwbfwbfwwzwgwgpgwwchcshslhssqsppqdqhhnssrzzvbbhjjqdjdgjgttvzzmvzvvnmvvfbbncbcqcpqpbbcjcncjnccjfjpjdjvdddbzzfrfwfpwpfwppvjvfjjhfhvvqttfhffscsvvbqbzqqjssbmsbshsrshsphhwhvwhwcwwnjjlzzfgfrgrwggfddpqpqqcmmtwtcwttgjjnfnnjvvmfmccvhchjcjmccmmwgghccnffjccqczqqbdbrdrsdsffmzzndnncbnbgbsshbhvbvsbvsbbvjbvbdvvqpqnpngpnggbbndbbdzdhzdhhjcjjmbbqgqbbfrrvzvcvffhggcbcjbbqtbbszbbvvcnvnccldddlrrtqrttpjpnjpnpznnqmnnrrmmsdmdbbffcwwllnffssspvvrjjwhwmmqhmmhjjtnjtjgttnrtnnsgsbssffstssjvjwwwwnjngjnnnlndndqdqqwdwcdcsccsgssgrrqwqqqhgqgfgffmnffzjjssqsfqqgddgcgtgrrvgrrdhdcdgdbdsswgwwpwggdfggnjnnvffzhhcqclqclcdlldbldlffdrrzttnhthdthhrthhgcgllzqlzzfhhmshssdvvtnnpngpprqrrrbnbssnhnnqdndsnshhzjjrnjnmjjwfffpbpnbbwddpwptwwpbpffvggmjmgghrrnqqhnhsnhsnnnlddmjddrdrhrchhgmgfmfdfcfpprsrzzzmhzhmhrmmrwmmpdmpmcclmlvvlffhlhhczccsvvptvpvwvdvqqdvqdddvnvsnswwspwpqqpddpvvsqsjjwnwrrjbjtttdhhhwbhbzzqrqfrrzlrzrqrbqrqzzfczfzrrqffnrnhrrcrtrvrffvflfsllcjcvvmqmlqqwjwgglhlvldltlqqdcddtssltsllsnndvdnddfqdffprrbssgccqmqrrnsrnnlzlhhqrhqhcqcjqcqpcqqmvqqgbgpbggnfggdpgpvpgvvsrrhqhshmsmwsmwssvnvtnttphpdpdspsvslshhlplmplmmljjvnnsrslrrvttqqprrptrptpnnmmflltntrrnjjcljcllwcllddbgbjbrrtwrtrftfctftsslsggggzqqhhrvvdqqwqbqtthdhrrpwpqpmmqcmqmllsmllhvfwbfvgzqgbhlfhqrqtzfpplgjtgngzrdfltnqlwsbhmwdfvrdjlgwftjvmdsgdgnswhrtmzgfqfsfnczjnmgqcfzvwlbgzsrpcbwwtrmbqtqhdmhmscqgjgpqdqcrvdvwplpdzsjqbvgpqcvfspqrcsjlhrqpjmdszjpqhmdwtddwqhbwsrlcjpzwsjjvbzcllqfwbhfvjqbqfbsrbgvgchdmgqjnvdrzlmrnlpzrljgjvtrdtqnzbnhpgpgjvwttmnfbpvrtmpstbmtwdwfzvznrwmspftgvrmdfwqltzzmlgrvvwjgdblnnbjzjfqpldsqbhrstnhrjqfcmzcrtqcpqmmfqzndgjwtcgnwrgdznzdgzbvmrlvrjvjgmfcrmrjbpjwqvhprbphnqsbpcpnflpgnpgggqpgrwghfpfvdljjqnvqgbvcpjbjlqghjppfhzfzczmcwnhrjzzrwlfqhvdwqrcbvprclnmqmqwdrhgtswwqhqtcjhndrmcrzdjnvwsstzplhcdzwzqbjjthsmjrmpfbqljlmnvmfddqflnhrfhchzgnlbcpvppnspsdtnqmfhrztznhltmqlrwzgczqmjggvnphwpvrwcnqhsdfglblvvdlqfnghtzgngtjnrzsjgwcglsrczlfqpmmzsrqwcclslzjpvpfgcwzwsfmwwswdsvnvtmwzmhwzvnnnldnjljbwlglpjvrggbmvhctsggtjgrmjrglnhtjzwhrvtmfmphgmpdmvlhzrhrtjcwswlnnjphsschrdvstmflzqsbqmqggjjdsdtjhcvjlzclbpvnbjgngnzvtdgszdtrwpnjmgwpcjpbsvrsmdjdjdbhqqvplbmfhmbgmtggvtltqjphlggscdbzncqhqqlmflmjbpnhbnqjcbfmggnftjjbzbrqwcgvfhsddrqgrvnztzzqzwjrjprmqctlttmgltdgrcvnbrbzrsbbqnsrslrswvmrqlctvsdfmcrdddcdwvfmwwcbqsdhmmgplzhnsfvhtctmvrzntvptpnbhhvjlphmtcctlhdvtvvqbmbczjqfvnqwzbgjzlgnqhvbjjnvrgfprvllltqdwvvmchrdqfczmmbmcjwtwgqbfjzgpcfzhhnwcghqltrlhlntjcrthpsbmcdzzqljdrdhfctwrzlvhdjvgmpscssmhdggrnhfbtfpqtsbqnqjfjwlbdfcpzfjtspnrjzshjcrrzwclddqqbplwghrcjhfqgblfrlrdhmdqmrzdnqthvvzlcjmqzqtgbcwcprpnjtbhsqmzlprhmssltfvvgqzzpnjtsmplhfpsnznjwrrgvmbbvjzzwctmwcwcwjwvmlpcmlmrfqbdjpclsqjnsnndzqfcghqsmhqcjwjjbbwmsttwnththhhgrlhrtrcjppvmjlqtqhpbhpsdhzcqpmqqbvwrlvjrnmlfvtntwghtphwzqmbmmpgvfrqsjwffdbcjjbwrrntrhnjwjfprzjcpnwvtwjcdhppspbdbwnzdbhvpqljbrfnpnbqdtqzdjlbvgbvvhcjrqwlqmhnhjfppllrsnsvnmttvrsdrqjfrdvhplwbhhnvglvgcvtzblfswzrrbwpjnslthjmgsdgwqrfjfplmbgsdcltzmdhrbpqggnjrcddltvdbrlhtzwfghbpnlmghwqdtpctmqbggfzflfzjgcbvfqtztsmgdvdlrrdrtmmjbtdtjvvjvlwwlwnzhlfqtrmdgvlflmbffnzlvtccnmcddrqfhgzlhngrfmsqdrnshshfhhtnwmnqmjtzggdfmpgcsrhjzhhvlppfnqmjsvtcwwjdvdrgrldbdnrqtvjjnddgmfdtqrncdlfznfpbqldztzzgpvwhvnqrrzdwsrdvcnslmwcdztvqzgbshpqrdbnmlfltpbsfrnbbcwvwvjnrdspwrwzpljqwwvqhbhhqqjnlrgwqvvsjrscbfqvhbwtjvwzmwzvftlwmtbmglvqvlnwrzvwclvzjcvgfstjpwdbtbhmnqzfgpqmthpchrrqfpqfflwhwrhmdzvgbzlmfgzvhrldprlgrvjcvjhslghqddfvqzgplmtfqjzhpctvtgrzjhwrjzbsqqrcztrstbvdjbtszrbvvzvwzfclsnrftcqtqdghdrnhglzptzwtqtgnfsqhmvnqqwtnwszgtvpnndbzcwgwhpcdlsdpptvlcqtmfhfjzbvfsvnjvqrrjqbggnjqfrrrvqmzscvlltfwzdlzqstlrrmqqmbmbtgbqjsvbncblddgzvlstmrpmrlfcqpvwthgbhlvfcfcwvslcjnztfwgdgwjfrwrzbbszmtvzchvstcfrgqllfrdccrjmtspshgqbzcldddqgjnpdsctmjphcwqvjtvmqlvvpzdgjdlpwdrjbshhrgjtglwnzlzsqngspbzgbwpgmfcv", 14);
  println!("Start message is at {}", start);
  assert!(start == 2250, "Wrong start seq");

}
