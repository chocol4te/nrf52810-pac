#[doc = "Pin select for RTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rts](rts) module"]
pub type RTS = crate::Reg<u32, _RTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTS;
#[doc = "`read()` method returns [rts::R](rts::R) reader structure"]
impl crate::Readable for RTS {}
#[doc = "`write(|w| ..)` method takes [rts::W](rts::W) writer structure"]
impl crate::Writable for RTS {}
#[doc = "Pin select for RTS"]
pub mod rts;
#[doc = "Pin select for TXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txd](txd) module"]
pub type TXD = crate::Reg<u32, _TXD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXD;
#[doc = "`read()` method returns [txd::R](txd::R) reader structure"]
impl crate::Readable for TXD {}
#[doc = "`write(|w| ..)` method takes [txd::W](txd::W) writer structure"]
impl crate::Writable for TXD {}
#[doc = "Pin select for TXD"]
pub mod txd;
#[doc = "Pin select for CTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cts](cts) module"]
pub type CTS = crate::Reg<u32, _CTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTS;
#[doc = "`read()` method returns [cts::R](cts::R) reader structure"]
impl crate::Readable for CTS {}
#[doc = "`write(|w| ..)` method takes [cts::W](cts::W) writer structure"]
impl crate::Writable for CTS {}
#[doc = "Pin select for CTS"]
pub mod cts;
#[doc = "Pin select for RXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxd](rxd) module"]
pub type RXD = crate::Reg<u32, _RXD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXD;
#[doc = "`read()` method returns [rxd::R](rxd::R) reader structure"]
impl crate::Readable for RXD {}
#[doc = "`write(|w| ..)` method takes [rxd::W](rxd::W) writer structure"]
impl crate::Writable for RXD {}
#[doc = "Pin select for RXD"]
pub mod rxd;
