#[doc = "Register `SS_EXT_TS_USIC` reader"]
pub type R = crate::R<SsExtTsUsicSpec>;
#[doc = "Register `SS_EXT_TS_USIC` writer"]
pub type W = crate::W<SsExtTsUsicSpec>;
#[doc = "Field `EXT_TS_INTR_CNTR` reader - 4:0\\]
Number of unserviced rollover interrupts. If >1 an EOI write will issue another pulse interrupt (EXT_TS_USIC - External TImeStamp Unserviced Interrupts Counter)"]
pub type ExtTsIntrCntrR = crate::FieldReader;
#[doc = "Field `EXT_TS_INTR_CNTR` writer - 4:0\\]
Number of unserviced rollover interrupts. If >1 an EOI write will issue another pulse interrupt (EXT_TS_USIC - External TImeStamp Unserviced Interrupts Counter)"]
pub type ExtTsIntrCntrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NU9` reader - 31:5\\]
Reserved"]
pub type Nu9R = crate::FieldReader<u32>;
#[doc = "Field `NU9` writer - 31:5\\]
Reserved"]
pub type Nu9W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Number of unserviced rollover interrupts. If >1 an EOI write will issue another pulse interrupt (EXT_TS_USIC - External TImeStamp Unserviced Interrupts Counter)"]
    #[inline(always)]
    pub fn ext_ts_intr_cntr(&self) -> ExtTsIntrCntrR {
        ExtTsIntrCntrR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Reserved"]
    #[inline(always)]
    pub fn nu9(&self) -> Nu9R {
        Nu9R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Number of unserviced rollover interrupts. If >1 an EOI write will issue another pulse interrupt (EXT_TS_USIC - External TImeStamp Unserviced Interrupts Counter)"]
    #[inline(always)]
    #[must_use]
    pub fn ext_ts_intr_cntr(&mut self) -> ExtTsIntrCntrW<SsExtTsUsicSpec> {
        ExtTsIntrCntrW::new(self, 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu9(&mut self) -> Nu9W<SsExtTsUsicSpec> {
        Nu9W::new(self, 5)
    }
}
#[doc = "SS_EXT_TS_USIC\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_ext_ts_usic::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_ext_ts_usic::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsExtTsUsicSpec;
impl crate::RegisterSpec for SsExtTsUsicSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_ext_ts_usic::R`](R) reader structure"]
impl crate::Readable for SsExtTsUsicSpec {}
#[doc = "`write(|w| ..)` method takes [`ss_ext_ts_usic::W`](W) writer structure"]
impl crate::Writable for SsExtTsUsicSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SS_EXT_TS_USIC to value 0"]
impl crate::Resettable for SsExtTsUsicSpec {
    const RESET_VALUE: u32 = 0;
}
