extern crate rand;
use self::rand::Rng;

use std::string::String;
use std::collections::HashMap;

pub fn map(input: String) -> String {
        let mut confusables = HashMap::new();
    
    confusables.insert('A', vec!['\u{1D400}','\u{1D434}','\u{1D5A0}','\u{1D5D4}','\u{1D608}','\u{1D63C}','\u{1D00}','\u{0391}','\u{1D6E2}','\u{1D71C}','\u{1D790}','\u{0410}','\u{13AA}','\u{A4EE}']);

    confusables.insert('B', vec!['\u{212C}','\u{1D401}','\u{1D435}','\u{1D469}','\u{1D4D1}','\u{1D505}','\u{1D609}','\u{1D63D}','\u{1D671}','\u{0392}','\u{1D6A9}','\u{1D6E3}','\u{1D71D}','\u{1D757}','\u{1D791}','\u{0412}','\u{13F4}','\u{15F7}','\u{A4D0}','\u{10282}','\u{102A1}','\u{10301}','\u{A7B4}','\u{0432}']);

    confusables.insert('C', vec!['\u{1D5A2}','\u{1D672}','\u{0421}','\u{0187}']);

    confusables.insert('D', vec!['\u{1D673}','\u{13A0}','\u{15DE}']);

    confusables.insert('E', vec!['\u{22FF}','\u{2130}','\u{1D404}','\u{1D438}','\u{1D46C}','\u{1D4D4}','\u{1D508}','\u{1D53C}','\u{1D570}','\u{1D5A4}','\u{1D5D8}','\u{1D60C}','\u{1D640}','\u{1D674}','\u{0395}','\u{1D6AC}','\u{1D6E6}','\u{1D720}','\u{1D75A}','\u{1D794}','\u{0415}','\u{2D39}','\u{13AC}','\u{A4F0}','\u{118A6}','\u{118AE}','\u{10286}','\u{011A}','\u{0246}','\u{2107}','\u{0510}','\u{13CB}','\u{10401}','\u{0190}']);

    confusables.insert('F', vec!['\u{2131}','\u{1D405}','\u{1D439}','\u{1D46D}','\u{1D4D5}','\u{1D509}','\u{1D53D}','\u{1D571}','\u{1D5A5}','\u{1D5D9}','\u{1D60D}','\u{1D641}','\u{1D675}','\u{A798}','\u{03DC}','\u{1D7CA}','\u{15B4}','\u{A4DD}','\u{118C2}','\u{118A2}','\u{10287}','\u{102A5}','\u{10525}','\u{0191}']);

    confusables.insert('G', vec!['\u{1D406}','\u{1D43A}','\u{1D46E}','\u{1D4A2}','\u{1D4D6}','\u{1D50A}','\u{1D53E}','\u{1D572}','\u{1D5A6}','\u{1D5DA}','\u{1D60E}','\u{1D642}','\u{1D676}','\u{050C}','\u{13C0}','\u{13F3}','\u{A4D6}','\u{01E6}','\u{011E}','\u{01E4}','\u{0193}','\u{0509}']);

    confusables.insert('H', vec!['\u{0397}','\u{041D}']);

    confusables.insert('I', vec!['\u{1D6B0}','\u{1D62D}','\u{0406}','\u{1D5A8}','\u{1D425}','\u{FE8D}','\u{FE8E}','\u{1D529}','\u{2111}','\u{1028A}','\u{2C92}','\u{10309}','\u{2113}','\u{1D724}','\u{0196}','\u{1D798}','\u{0399}','\u{1D695}','\u{1D7CF}','\u{2223}','\u{0627}','\u{1D5C5}','\u{1D540}','\u{0031}','\u{1D644}','\u{1D4C1}','\u{10320}','\u{1EE00}','\u{1EE80}','\u{05C0}','\u{1D470}','\u{01C0}','\u{16C1}','\u{1D7ED}','\u{1D574}','\u{07CA}','\u{1D6EA}','\u{2D4F}','\u{1D75E}','\u{1D55D}','\u{1D7E3}','\u{05D5}','\u{1E8C7}','\u{1D661}','\u{1D4D8}','\u{1D5DC}','\u{1D459}','\u{05DF}','\u{2160}','\u{1D610}','\u{0661}','\u{1D48D}','\u{1D591}','\u{FFE8}','\u{1D408}','\u{006C}','\u{06F1}','\u{A4F2}','\u{16F28}','\u{1D678}','\u{1D7F7}','\u{1D4F5}','\u{007C}','\u{217C}','\u{23FD}','\u{1D5F9}']);

    confusables.insert('J', vec!['\u{1D409}','\u{1D43D}','\u{1D471}','\u{1D4A5}','\u{1D4D9}','\u{1D50D}','\u{1D541}','\u{1D575}','\u{1D5A9}','\u{1D5DD}','\u{1D611}','\u{1D645}','\u{1D679}','\u{037F}','\u{0408}','\u{13AB}','\u{148D}','\u{A4D9}','\u{A7B2}','\u{0248}','\u{1499}','\u{0575}','\u{1D6A5}']);

    confusables.insert('K', vec!['\u{212A}','\u{1D40A}','\u{1D43E}','\u{1D472}','\u{1D4A6}','\u{1D4DA}','\u{1D50A}','\u{1D542}','\u{1D576}','\u{1D5AA}','\u{1D5DE}','\u{1D612}','\u{1D646}','\u{1D67A}','\u{039A}','\u{1D6B1}','\u{1D6EB}','\u{1D725}','\u{1D75F}','\u{1D799}','\u{2C94}','\u{041A}','\u{13E6}','\u{16D5}','\u{A4D7}','\u{10518}','\u{2C69}','\u{049A}','\u{049E}','\u{0198}']);

    confusables.insert('L', vec!['\u{216C}','\u{2112}','\u{1D408}','\u{1D43F}','\u{1D473}','\u{1D4DB}','\u{1D50F}','\u{1D543}','\u{1D577}','\u{1D5AB}','\u{1D5DF}','\u{1D613}','\u{1D647}','\u{1D67B}','\u{2CD0}','\u{13DE}','\u{14AA}','\u{A4E1}','\u{118A3}','\u{118B2}','\u{1041B}','\u{10526}','\u{0141}','\u{14B7}','\u{013F}']);

    confusables.insert('M', vec!['\u{216F}','\u{2133}','\u{1D40C}','\u{1D440}','\u{1D474}','\u{1D4DC}','\u{1D510}','\u{1D544}','\u{1D578}','\u{1D5AC}','\u{1D5E0}','\u{1D614}','\u{1D648}','\u{1D67C}','\u{039C}','\u{1D6B3}','\u{1D6ED}','\u{1D727}','\u{1D761}','\u{1D79B}','\u{03FA}','\u{2C98}','\u{041C}','\u{13B7}','\u{15F0}','\u{16D6}','\u{A4DF}','\u{102B0}','\u{10311}','\u{04CD}']);

    confusables.insert('N', vec!['\u{2115}','\u{1D40D}','\u{1D441}','\u{1D475}','\u{1D4A9}','\u{1D4DD}','\u{1D511}','\u{1D579}','\u{1D5AD}','\u{1D5E1}','\u{1D615}','\u{1D649}','\u{1D67D}','\u{039D}','\u{1D6B4}','\u{1D6EE}','\u{1D728}','\u{1D762}','\u{1D79C}','\u{2C9A}','\u{A4E0}','\u{10513}','\u{019D}']);

    confusables.insert('O', vec!['\u{0A66}','\u{0AE6}','\u{0BE6}','\u{0C66}','\u{0CE6}','\u{0ED0}','\u{1040}','\u{0030}','\u{07C0}','\u{09E6}','\u{0B66}','\u{3007}','\u{114D0}','\u{118E0}','\u{1D7CE}','\u{1D7D8}','\u{1D7E2}','\u{1D7EC}','\u{1D7F6}','\u{1D40E}','\u{1D442}','\u{1D476}','\u{1D4AA}','\u{1D4DE}','\u{1D512}','\u{1D546}','\u{1D57A}','\u{1D5AE}','\u{1D5E2}','\u{1D616}','\u{1D64A}','\u{1D67E}','\u{039F}','\u{1D6B6}','\u{1D6F0}','\u{1D72A}','\u{1D764}','\u{1D79E}','\u{2C9E}','\u{041E}','\u{0555}','\u{2D54}','\u{0B20}','\u{0D20}','\u{A4F3}','\u{118B5}','\u{10292}','\u{102AB}','\u{10404}','\u{10516}','\u{01D1}','\u{00D8}','\u{2D41}','\u{01FE}','\u{2296}','\u{229D}','\u{1F100}','\u{1F101}','\u{01A0}','\u{13A4}']);

    confusables.insert('P', vec!['\u{2119}','\u{1D40F}','\u{1D443}','\u{1D477}','\u{1D4AB}','\u{1D4DF}','\u{1D513}','\u{1D57B}','\u{1D5AF}','\u{1D5E3}','\u{1D617}','\u{1D64B}','\u{1D67F}','\u{03A1}','\u{1D6B8}','\u{1D6F2}','\u{1D72C}','\u{1D766}','\u{1D7A0}','\u{2CA2}','\u{0420}','\u{13E2}','\u{146D}','\u{A4D1}','\u{10295}','\u{1477}','\u{1486}',]);

    confusables.insert('Q', vec!['\u{211A}','\u{1D410}','\u{1D444}','\u{1D478}','\u{1D4AC}','\u{1D4E0}','\u{1D514}','\u{1D57C}','\u{1D5B0}','\u{1D5E4}','\u{1D618}','\u{1D64C}','\u{1D680}','\u{2D55}']);

    confusables.insert('R', vec!['\u{211B}','\u{211C}','\u{211D}','\u{1D411}','\u{1D445}','\u{1D479}','\u{1D4E1}','\u{1D57D}','\u{1D5B1}','\u{1D5E5}','\u{1D619}','\u{1D64D}','\u{1D681}','\u{01A6}','\u{13A1}','\u{13D2}','\u{1587}','\u{A4E3}']);

    confusables.insert('S', vec!['\u{1D412}','\u{1D446}','\u{1D47A}','\u{1D4AE}','\u{1D4E2}','\u{1D516}','\u{1D54A}','\u{1D57E}','\u{1D5B2}','\u{1D5E6}','\u{1D61A}','\u{1D64E}','\u{1D682}','\u{0405}','\u{054F}','\u{13D5}','\u{13DA}','\u{A4E2}','\u{10296}','\u{10420}']);

    confusables.insert('T', vec!['\u{22A4}','\u{27D9}','\u{1F768}','\u{1D413}','\u{1D447}','\u{1D47B}','\u{1D4AF}','\u{1D4E3}','\u{1D517}','\u{1D548}','\u{1D57F}','\u{1D5B3}','\u{1D5E7}','\u{1D61B}','\u{1D64F}','\u{1D683}','\u{03A4}','\u{1D6BB}','\u{1D6F5}','\u{1D72F}','\u{1D769}','\u{1D7A3}','\u{2CA6}','\u{0422}','\u{13A2}','\u{A4D4}','\u{118BC}','\u{10297}','\u{102B1}','\u{10315}','\u{2361}','\u{023E}','\u{021A}','\u{01AE}','\u{04AC}','\u{20AE}','\u{0166}']);

    confusables.insert('U', vec!['\u{222A}','\u{22C3}','\u{1D414}','\u{1D448}','\u{1D47C}','\u{1D4B0}','\u{1D4E4}','\u{1D518}','\u{1D54C}','\u{1D580}','\u{1D5B4}','\u{1D5E8}','\u{1D61C}','\u{1D650}','\u{1D684}','\u{054D}','\u{144C}','\u{A4F4}','\u{118B8}','\u{01D3}','\u{0244}','\u{13CC}','\u{1458}','\u{1467}','\u{01B1}','\u{162E}']);

    confusables.insert('V', vec!['\u{2228}','\u{22C1}','\u{0667}','\u{06F7}','\u{2164}','\u{1D415}','\u{1D449}','\u{1D47D}','\u{1D4B1}','\u{1D4E5}','\u{1D519}','\u{1D54D}','\u{1D581}','\u{1D5B5}','\u{1D5E9}','\u{1D61D}','\u{1D651}','\u{1D685}','\u{0474}','\u{2D38}','\u{13D9}','\u{142F}','\u{A4E6}','\u{118A0}','\u{1051D}','\u{143B}','\u{1F708}']);

    confusables.insert('W', vec!['\u{118EF}','\u{118E6}','\u{1D416}','\u{1D44A}','\u{1D47E}','\u{1D4B2}','\u{1D4E6}','\u{1D51A}','\u{1D54E}','\u{1D582}','\u{1D5B6}','\u{1D5EA}','\u{1D61E}','\u{1D652}','\u{1D686}','\u{051C}','\u{13B3}','\u{13D4}','\u{A4EA}','\u{20A9}']);

    confusables.insert('X', vec!['\u{166D}','\u{2573}','\u{10322}','\u{118EC}','\u{2169}','\u{1D417}','\u{1D44B}','\u{1D47F}','\u{1D4B3}','\u{1D4E7}','\u{1D51B}','\u{1D54F}','\u{1D583}','\u{1D5B7}','\u{1D5EB}','\u{1D61F}','\u{1D653}','\u{1D687}','\u{03A7}','\u{1D6BE}','\u{1D6F8}','\u{1D732}','\u{1D76C}','\u{1D7A6}','\u{2CAC}','\u{0425}','\u{2D5D}','\u{2D5D}','\u{16B7}','\u{A4EB}','\u{10290}','\u{102B4}','\u{10317}','\u{10527}','\u{A7B3}','\u{04B2}']);

    confusables.insert('Y', vec!['\u{1D418}','\u{1D44C}','\u{1D480}','\u{1D4B4}','\u{1D4E8}','\u{1D51C}','\u{1D550}','\u{1D584}','\u{1D5B8}','\u{1D5EC}','\u{1D620}','\u{1D654}','\u{1D688}','\u{03A5}','\u{03D2}','\u{1D6BC}','\u{1D6F6}','\u{1D730}','\u{1D76A}','\u{1D7A4}','\u{2CA8}','\u{04AE}','\u{13A9}','\u{13BD}','\u{A4EC}','\u{118A4}','\u{102B2}','\u{00A5}','\u{024E}','\u{04B0}']);

    confusables.insert('Z', vec!['\u{102F5}','\u{118E5}','\u{2124}','\u{2128}','\u{1D419}','\u{1D44D}','\u{1D481}','\u{1D4B5}','\u{1D4E9}','\u{1D585}','\u{1D5B9}','\u{1D5ED}','\u{1D621}','\u{1D655}','\u{1D689}','\u{0396}','\u{1D6AD}','\u{1D6E7}','\u{1D721}','\u{1D75B}','\u{1D795}','\u{13C3}','\u{A4DC}','\u{118A9}','\u{01B5}','\u{0224}']);

    confusables.insert('a', vec!['\u{237a}','\u{1D41A}','\u{1D44E}','\u{1D482}','\u{1D4B6}','\u{1D4EA}','\u{1D51E}','\u{1D552}','\u{1D586}','\u{1D5BA}','\u{1D5EE}','\u{1D622}','\u{1D656}','\u{1D68A}','\u{0251}','\u{03B1}','\u{1D6C2}','\u{1D6FC}','\u{1D736}','\u{1D770}','\u{1D7AA}','\u{0430}','\u{2376}','\u{0103}','\u{01CE}','\u{0227}','\u{00E5}','\u{1E9A}','\u{1EA3}']);

    confusables.insert('b', vec!['\u{1D41B}','\u{1D44F}','\u{1D483}','\u{1D4B7}','\u{1D4EB}','\u{1D51F}','\u{1D553}','\u{1D587}','\u{1D5BB}','\u{1D5EF}','\u{1D623}','\u{1D657}','\u{1D68B}','\u{0184}','\u{042C}','\u{13CF}','\u{15AF}','\u{0253}','\u{0183}','\u{0411}','\u{0182}','\u{0180}','\u{048C}','\u{048D}','\u{0462}','\u{0463}']);

    confusables.insert('c', vec!['\u{217D}','\u{1D41C}','\u{1D450}','\u{1D484}','\u{1D4B8}','\u{1D4EC}','\u{1D520}','\u{1D554}','\u{1D588}','\u{1D5BC}','\u{1D5F0}','\u{1D624}','\u{1D658}','\u{1D68C}','\u{1D04}','\u{03F2}','\u{2CA5}','\u{0441}','\u{1043D}','\u{00A2}','\u{023C}','\u{00E7}','\u{04AB}']);

    confusables.insert('d', vec!['\u{217E}','\u{2146}','\u{1D41D}','\u{1D451}','\u{1D485}','\u{1D4B9}','\u{1D4ED}','\u{1D521}','\u{1D555}','\u{1D589}','\u{1D5BD}','\u{1D5F1}','\u{1D625}','\u{1D659}','\u{1D68D}','\u{0501}','\u{13E7}','\u{146F}','\u{A4D2}','\u{0257}','\u{0256}','\u{018}','\u{0111}','\u{20AB}','\u{147B}','\u{1487}']);

    confusables.insert('e', vec!['\u{1D41E}','\u{0435}']);

    confusables.insert('f', vec!['\u{1E9D}','\u{0192}']);

    confusables.insert('g', vec!['\u{210A}','\u{1D420}','\u{1D454}','\u{1D488}','\u{1D4F0}','\u{1D524}','\u{1D558}','\u{1D58C}','\u{1D5C0}','\u{1D5F4}','\u{1D628}','\u{1D65C}','\u{1D690}','\u{0261}','\u{1D83}','\u{018D}','\u{0581}','\u{0260}','\u{01E7}','\u{01F5}','\u{01E5}']);

    confusables.insert('h', vec!['\u{210E}','\u{1D421}','\u{1D489}','\u{1D4BD}','\u{1D4F1}','\u{1D525}','\u{1D559}','\u{1D58D}','\u{1D5C1}','\u{1D5F5}','\u{1D629}','\u{1D65D}','\u{1D691}','\u{04BB}','\u{0570}','\u{13C2}','\u{0266}','\u{A695}','\u{13F2}','\u{0127}','\u{210F}','\u{045B}']);

    confusables.insert('i', vec!['\u{02DB}','\u{2373}','\u{2170}','\u{2139}','\u{2148}','\u{1D422}','\u{1D456}','\u{1D48A}','\u{1D4BE}','\u{1D4F2}','\u{1D526}','\u{1D55A}','\u{1D58E}','\u{1D5C2}','\u{1D5F6}','\u{1D62A}','\u{1D65E}','\u{1D692}','\u{0131}','\u{1D6A4}','\u{026A}','\u{0269}','\u{03B9}','\u{1FBE}','\u{037A}','\u{1D6CA}','\u{1D704}','\u{1D73E}','\u{1D778}','\u{1D7B2}','\u{0456}','\u{A647}','\u{04CF}','\u{13A5}','\u{118C3}','\u{2378}','\u{01D0}','\u{0268}','\u{1D7C}']);

    confusables.insert('j', vec!['\u{2149}','\u{1D423}','\u{1D457}','\u{1D48B}','\u{1D4BF}','\u{1D4F3}','\u{1D527}','\u{1D55B}','\u{1D58F}','\u{1D5C3}','\u{1D5F7}','\u{1D62B}','\u{1D65F}','\u{1D693}','\u{03F3}','\u{0458}','\u{0249}','\u{1D6A5}','\u{0237}','\u{0575}']);

    confusables.insert('k', vec!['\u{1D424}','\u{1D458}','\u{1D48C}','\u{1D4C0}','\u{1D4F4}','\u{1D528}','\u{1D55C}','\u{1D590}','\u{1D5C4}','\u{1D4F8}','\u{1D62C}','\u{1D660}','\u{1D694}','\u{1D0B}','\u{0138}','\u{03BA}','\u{03F0}','\u{1D6CB}','\u{1D6DE}','\u{1D705}','\u{1D718}','\u{1D73F}','\u{1D752}','\u{1D779}','\u{1D78C}','\u{1D7B3}','\u{1D7C6}','\u{2C95}','\u{043A}','\u{0199}','\u{049B}']);

    confusables.insert('l', vec!['\u{2223}','\u{0661}','\u{06F1}']);

    confusables.insert('m', vec!['\u{11700}','\u{1D48E}','\u{1D62E}','\u{118E3}','\u{1D592}','\u{1D426}','\u{1D5C6}','\u{1D52A}','\u{1D55E}','\u{1D4C2}','\u{1D662}','\u{1D4F6}','\u{1D696}','\u{1D45A}','\u{1D5FA}','\u{217F}']);

    confusables.insert('n', vec!['\u{03C0}','\u{043F}']);

    confusables.insert('o', vec!['\u{0966}','\u{0BE6}','\u{0D66}','\u{0E50}','\u{1040}']);

    confusables.insert('p', vec!['\u{2374}','\u{1D429}','\u{1D45D}','\u{1D491}','\u{1D4C5}','\u{1D4F9}','\u{1D52D}','\u{1D561}','\u{1D595}','\u{1D5C9}','\u{1D5FD}','\u{1D631}','\u{1D665}','\u{1D699}','\u{03C1}','\u{03F1}','\u{1D6D2}','\u{1D6E0}','\u{1D70C}','\u{1D71A}','\u{1D746}','\u{1D754}','\u{1D780}','\u{1D78E}','\u{1D7BA}','\u{1D7C8}','\u{2CA3}','\u{0440}','\u{01A5}','\u{1D7D}']);

    confusables.insert('q', vec!['\u{1D42A}','\u{1D45E}','\u{1D492}','\u{1D4C6}','\u{1D4FA}','\u{1D52E}','\u{1D562}','\u{1D596}','\u{1D5CA}','\u{1D5FE}','\u{1D632}','\u{1D666}','\u{1D69A}','\u{051B}','\u{0563}','\u{0566}','\u{02A0}','\u{1D90}','\u{024B}']);

    confusables.insert('r', vec!['\u{1D42B}','\u{1D45F}','\u{1D493}','\u{1D4C7}','\u{1D4FB}','\u{1D52F}','\u{1D563}','\u{1D597}','\u{1D5CB}','\u{1D5FF}','\u{1D633}','\u{1D667}','\u{1D69B}','\u{AB47}','\u{AB48}','\u{1D26}','\u{2C85}','\u{0433}','\u{027D}','\u{027C}','\u{024D}','\u{0493}','\u{1D72}','\u{0491}']);

    confusables.insert('s', vec!['\u{1D5CC}','\u{1D600}','\u{1D634}','\u{1D69C}','\u{0455}','\u{0282}']);

    confusables.insert('t', vec!['\u{1D42D}','\u{1D461}','\u{1D495}','\u{1D4C9}','\u{1D4FD}','\u{1D531}','\u{1D565}','\u{1D599}','\u{1D5CD}','\u{1D601}','\u{1D635}','\u{1D669}','\u{1D69D}','\u{1D1B}','\u{03C4}','\u{1D6D5}','\u{1D70F}','\u{1D749}','\u{1D783}','\u{1D7BD}','\u{0442}','\u{01AD}','\u{04AD}','\u{0167}','\u{0163}','\u{021b}']);

    confusables.insert('u', vec!['\u{1D42E}','\u{1D462}','\u{1D496}','\u{1D4CA}','\u{1D4FE}','\u{1D532}','\u{1D566}','\u{1D59A}','\u{1D5CE}','\u{1D602}','\u{1D636}','\u{1D66A}','\u{1D69E}','\u{A79F}','\u{1D1C}','\u{AB4E}','\u{1B52}','\u{028B}','\u{03C5}','\u{1D6D6}','\u{1D710}','\u{1D74A}','\u{1D784}','\u{1D7BE}','\u{0446}','\u{057D}','\u{118D8}','\u{01D4}','\u{197E}','\u{028A}']);

    confusables.insert('v', vec!['\u{2228}','\u{22C1}','\u{2174}','\u{1D42F}','\u{1D463}','\u{1D497}','\u{1D4CB}','\u{1D4FF}','\u{1D533}','\u{1D567}','\u{1D59B}','\u{1D5CF}','\u{1D603}','\u{1D637}','\u{1D66B}','\u{1D69F}','\u{1D20}','\u{03BD}','\u{1D6CE}','\u{1D708}','\u{1D742}','\u{1D77C}','\u{1D7B6}','\u{0475}','\u{05DB}','\u{118C0}']);

    confusables.insert('w', vec!['\u{1D430}','\u{1D5D0}','\u{1D21}','\u{0461}','\u{0561}','\u{AB83}','\u{1D534}','\u{1D568}','\u{1D4CC}','\u{1D66C}','\u{026F}','\u{1D500}','\u{1D6A0}','\u{1170F}','\u{1170E}','\u{1D464}','\u{1D604}','\u{1D498}','\u{1D638}','\u{1D59C}','\u{051D}','\u{1170A}']);

    confusables.insert('x', vec!['\u{166E}','\u{00D7}','\u{292B}','\u{292C}','\u{2A2F}','\u{2179}','\u{1D431}','\u{1D465}','\u{1D499}','\u{1D4CD}','\u{1D501}','\u{1D535}','\u{1D569}','\u{1D59D}','\u{1D5D1}','\u{1D605}','\u{1D639}','\u{1D66D}','\u{1D6A1}','\u{0445}','\u{1541}','\u{157D}','\u{2A30}']);

    confusables.insert('y', vec!['\u{02C3}','\u{1D8C}','\u{1D432}','\u{1D466}','\u{1D49A}','\u{1D4CE}','\u{1D502}','\u{1D536}','\u{1D56A}','\u{1D59E}','\u{1D5D2}','\u{1D606}','\u{1D63A}','\u{1D66E}','\u{1D6A2}','\u{028F}','\u{1EFF}','\u{AB5A}','\u{03B3}','\u{213D}','\u{1D6C4}','\u{1D6FE}','\u{1D738}','\u{1D772}','\u{1D7AC}','\u{0443}','\u{04AF}','\u{10E7}','\u{118DC}','\u{01B4}','\u{024F}','\u{04B1}']);

    confusables.insert('z', vec!['\u{1D433}','\u{1D467}','\u{1D49B}','\u{1D4CF}','\u{1D503}','\u{1D537}','\u{1D56B}','\u{1D59F}','\u{1D5D3}','\u{1D607}','\u{1D63B}','\u{1D66F}','\u{1D6A3}','\u{1D22}','\u{118C4}','\u{0290}','\u{01B6}','\u{0225}','\u{1D76}']);


    let mut output = String::new();
    let mut input_chars = input.chars().peekable();
    while input_chars.peek() != None {
        let next = input_chars.next().unwrap();
        if rand::thread_rng().gen_range(0,10) > 0 {
            output.push(next);
            continue;
        }

        let next_confusables = confusables.get(&next);
        //TODO: do I want to make homograph density configurable? Do science to figure out the
        //      right threashold?
        match next_confusables {
            Some(next_confusables) => {
                let next_out = rand::thread_rng().choose(next_confusables);
                println!("mapping char {} to unicode point {}", next, next_out.unwrap().escape_unicode());
                output.push(*next_out.unwrap());
            }    
            None => {
                output.push(next);
                continue;
            }
        }
    }
    return output;
}