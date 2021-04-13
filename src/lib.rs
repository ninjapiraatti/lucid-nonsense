pub mod rng;
pub mod world;
pub mod plants;
pub mod utils;

lazy_static::lazy_static! {
	static ref CHARS_ALL: String = String::from("ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■ ☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼!\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂");
	static ref CHARS_HRZ: String = String::from("⌐¬½¼┴┬─═╧╨╤±≥≤↔-=");
	static ref CHARS_VRT: String = String::from("│┤╡╢╣║┼╞╟╠╬╫╪⌡|");
	static ref CHARS_RND: String = String::from("ÇêëèôöÖóúªº¿ßσµΦΘΩδ∞°²☺☻♥♦♣♠•○♂♀☼%0@");
	static ref SIZE: (u16, u16) = termion::terminal_size().unwrap();
}
