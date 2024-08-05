#[doc = "Register `LINMASK` reader"]
pub type R = crate::R<LinmaskSpec>;
#[doc = "Register `LINMASK` writer"]
pub type W = crate::W<LinmaskSpec>;
#[doc = "Field `TXIDMASK` reader - 7:0\\]
Transmit ID mask. This field is effective in LIN mode only. This 8-bit mask is used for filtering an incoming ID message and compare it to the ID-byte. A compare match of the received ID with the TX ID Mask will set the ID TX flag and trigger an ID interrupt if enabled. A 0 bit in the mask indicates that bit is compared to the ID-byte. A 1 bit in the mask indicates that bit is filtered and therefore not used for the compare. When HGENCTRL is set to 1, this field must be set to 0xFF."]
pub type TxidmaskR = crate::FieldReader;
#[doc = "Field `TXIDMASK` writer - 7:0\\]
Transmit ID mask. This field is effective in LIN mode only. This 8-bit mask is used for filtering an incoming ID message and compare it to the ID-byte. A compare match of the received ID with the TX ID Mask will set the ID TX flag and trigger an ID interrupt if enabled. A 0 bit in the mask indicates that bit is compared to the ID-byte. A 1 bit in the mask indicates that bit is filtered and therefore not used for the compare. When HGENCTRL is set to 1, this field must be set to 0xFF."]
pub type TxidmaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RXIDMASK` reader - 23:16\\]
Receive ID mask. This field is effective in LIN mode only.This 8-bit mask is used for filtering an incoming ID message and compare it to the ID-byte. A compare match of the received ID witht the RX ID mask will set the ID RX flag and trigger and ID interrupt if enabled. A 0 bit in the mask indicates that bit is compared to the ID-byte. A 1 bit in the mask indicates that that bit is filtered and therefore not used in the compare. When HGENCTRL is set to 1, this field must be set to 0xFF."]
pub type RxidmaskR = crate::FieldReader;
#[doc = "Field `RXIDMASK` writer - 23:16\\]
Receive ID mask. This field is effective in LIN mode only.This 8-bit mask is used for filtering an incoming ID message and compare it to the ID-byte. A compare match of the received ID witht the RX ID mask will set the ID RX flag and trigger and ID interrupt if enabled. A 0 bit in the mask indicates that bit is compared to the ID-byte. A 1 bit in the mask indicates that that bit is filtered and therefore not used in the compare. When HGENCTRL is set to 1, this field must be set to 0xFF."]
pub type RxidmaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved1` reader - 31:24\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 31:24\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Transmit ID mask. This field is effective in LIN mode only. This 8-bit mask is used for filtering an incoming ID message and compare it to the ID-byte. A compare match of the received ID with the TX ID Mask will set the ID TX flag and trigger an ID interrupt if enabled. A 0 bit in the mask indicates that bit is compared to the ID-byte. A 1 bit in the mask indicates that bit is filtered and therefore not used for the compare. When HGENCTRL is set to 1, this field must be set to 0xFF."]
    #[inline(always)]
    pub fn txidmask(&self) -> TxidmaskR {
        TxidmaskR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Receive ID mask. This field is effective in LIN mode only.This 8-bit mask is used for filtering an incoming ID message and compare it to the ID-byte. A compare match of the received ID witht the RX ID mask will set the ID RX flag and trigger and ID interrupt if enabled. A 0 bit in the mask indicates that bit is compared to the ID-byte. A 1 bit in the mask indicates that that bit is filtered and therefore not used in the compare. When HGENCTRL is set to 1, this field must be set to 0xFF."]
    #[inline(always)]
    pub fn rxidmask(&self) -> RxidmaskR {
        RxidmaskR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Transmit ID mask. This field is effective in LIN mode only. This 8-bit mask is used for filtering an incoming ID message and compare it to the ID-byte. A compare match of the received ID with the TX ID Mask will set the ID TX flag and trigger an ID interrupt if enabled. A 0 bit in the mask indicates that bit is compared to the ID-byte. A 1 bit in the mask indicates that bit is filtered and therefore not used for the compare. When HGENCTRL is set to 1, this field must be set to 0xFF."]
    #[inline(always)]
    #[must_use]
    pub fn txidmask(&mut self) -> TxidmaskW<LinmaskSpec> {
        TxidmaskW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Receive ID mask. This field is effective in LIN mode only.This 8-bit mask is used for filtering an incoming ID message and compare it to the ID-byte. A compare match of the received ID witht the RX ID mask will set the ID RX flag and trigger and ID interrupt if enabled. A 0 bit in the mask indicates that bit is compared to the ID-byte. A 1 bit in the mask indicates that that bit is filtered and therefore not used in the compare. When HGENCTRL is set to 1, this field must be set to 0xFF."]
    #[inline(always)]
    #[must_use]
    pub fn rxidmask(&mut self) -> RxidmaskW<LinmaskSpec> {
        RxidmaskW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<LinmaskSpec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "The LINMASK register is used to configure the masks used for filtering incoming ID messages for receive and transmit frames.\n\nYou can [`read`](crate::Reg::read) this register and get [`linmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinmaskSpec;
impl crate::RegisterSpec for LinmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`linmask::R`](R) reader structure"]
impl crate::Readable for LinmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`linmask::W`](W) writer structure"]
impl crate::Writable for LinmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINMASK to value 0"]
impl crate::Resettable for LinmaskSpec {
    const RESET_VALUE: u32 = 0;
}
