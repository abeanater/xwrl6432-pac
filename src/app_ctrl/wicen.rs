#[doc = "Register `WICEN` reader"]
pub type R = crate::R<WicenSpec>;
#[doc = "Register `WICEN` writer"]
pub type W = crate::W<WicenSpec>;
#[doc = "Field `wicen` reader - 0:0\\]
1 :> Wakeup Interrupt Controller (WIC) of CM4 is Enabled 0 :> Disabled"]
pub type WicenR = crate::BitReader;
#[doc = "Field `wicen` writer - 0:0\\]
1 :> Wakeup Interrupt Controller (WIC) of CM4 is Enabled 0 :> Disabled"]
pub type WicenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1 :> Wakeup Interrupt Controller (WIC) of CM4 is Enabled 0 :> Disabled"]
    #[inline(always)]
    pub fn wicen(&self) -> WicenR {
        WicenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1 :> Wakeup Interrupt Controller (WIC) of CM4 is Enabled 0 :> Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn wicen(&mut self) -> WicenW<WicenSpec> {
        WicenW::new(self, 0)
    }
}
#[doc = "WICEN\n\nYou can [`read`](crate::Reg::read) this register and get [`wicen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wicen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WicenSpec;
impl crate::RegisterSpec for WicenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wicen::R`](R) reader structure"]
impl crate::Readable for WicenSpec {}
#[doc = "`write(|w| ..)` method takes [`wicen::W`](W) writer structure"]
impl crate::Writable for WicenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WICEN to value 0"]
impl crate::Resettable for WicenSpec {
    const RESET_VALUE: u32 = 0;
}
