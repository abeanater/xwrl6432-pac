#[doc = "Register `RTIWDKEY` reader"]
pub type R = crate::R<RtiwdkeySpec>;
#[doc = "Register `RTIWDKEY` writer"]
pub type W = crate::W<RtiwdkeySpec>;
#[doc = "Field `WDKEY` reader - 15:0\\]
WDKEY: Watchdog Key. User and privilege mode reads are indeterminate. Privilege mode (write): A write of 0xE51A followed by 0xA35C in two separate write operations defines the Key Sequence and discharges the watchdog capacitor. This also causes the upper 12 bits of the DWD down counter to be reloaded with the contents of the DWD preload register and the lower 13 bits to become all 1ΓÇÖs. Writing any other value causes a digital watchdog reset, as shown in Table 1-3. Note: Register write access time precaution The user has to take into account that the write to the register takes 3 VCLK cycle. This needs to be considered for the AWD/DWD expiration calculation."]
pub type WdkeyR = crate::FieldReader<u16>;
#[doc = "Field `WDKEY` writer - 15:0\\]
WDKEY: Watchdog Key. User and privilege mode reads are indeterminate. Privilege mode (write): A write of 0xE51A followed by 0xA35C in two separate write operations defines the Key Sequence and discharges the watchdog capacitor. This also causes the upper 12 bits of the DWD down counter to be reloaded with the contents of the DWD preload register and the lower 13 bits to become all 1ΓÇÖs. Writing any other value causes a digital watchdog reset, as shown in Table 1-3. Note: Register write access time precaution The user has to take into account that the write to the register takes 3 VCLK cycle. This needs to be considered for the AWD/DWD expiration calculation."]
pub type WdkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED19` reader - 31:16\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved19R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED19` writer - 31:16\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved19W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
WDKEY: Watchdog Key. User and privilege mode reads are indeterminate. Privilege mode (write): A write of 0xE51A followed by 0xA35C in two separate write operations defines the Key Sequence and discharges the watchdog capacitor. This also causes the upper 12 bits of the DWD down counter to be reloaded with the contents of the DWD preload register and the lower 13 bits to become all 1ΓÇÖs. Writing any other value causes a digital watchdog reset, as shown in Table 1-3. Note: Register write access time precaution The user has to take into account that the write to the register takes 3 VCLK cycle. This needs to be considered for the AWD/DWD expiration calculation."]
    #[inline(always)]
    pub fn wdkey(&self) -> WdkeyR {
        WdkeyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved19(&self) -> Reserved19R {
        Reserved19R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
WDKEY: Watchdog Key. User and privilege mode reads are indeterminate. Privilege mode (write): A write of 0xE51A followed by 0xA35C in two separate write operations defines the Key Sequence and discharges the watchdog capacitor. This also causes the upper 12 bits of the DWD down counter to be reloaded with the contents of the DWD preload register and the lower 13 bits to become all 1ΓÇÖs. Writing any other value causes a digital watchdog reset, as shown in Table 1-3. Note: Register write access time precaution The user has to take into account that the write to the register takes 3 VCLK cycle. This needs to be considered for the AWD/DWD expiration calculation."]
    #[inline(always)]
    #[must_use]
    pub fn wdkey(&mut self) -> WdkeyW<RtiwdkeySpec> {
        WdkeyW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved19(&mut self) -> Reserved19W<RtiwdkeySpec> {
        Reserved19W::new(self, 16)
    }
}
#[doc = "Watchdog Key correct written key values discharge the external capacitor\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiwdkey::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiwdkey::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtiwdkeySpec;
impl crate::RegisterSpec for RtiwdkeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtiwdkey::R`](R) reader structure"]
impl crate::Readable for RtiwdkeySpec {}
#[doc = "`write(|w| ..)` method takes [`rtiwdkey::W`](W) writer structure"]
impl crate::Writable for RtiwdkeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTIWDKEY to value 0"]
impl crate::Resettable for RtiwdkeySpec {
    const RESET_VALUE: u32 = 0;
}
