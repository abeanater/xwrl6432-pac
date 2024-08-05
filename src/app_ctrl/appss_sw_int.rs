#[doc = "Register `APPSS_SW_INT` reader"]
pub type R = crate::R<AppssSwIntSpec>;
#[doc = "Register `APPSS_SW_INT` writer"]
pub type W = crate::W<AppssSwIntSpec>;
#[doc = "Field `pulse` reader - 3:0\\]
Write_pulse bit field: writing 1'b1 to each bit will trigger SW_INT&lt;0-3> respectively to CM4."]
pub type PulseR = crate::FieldReader;
#[doc = "Field `pulse` writer - 3:0\\]
Write_pulse bit field: writing 1'b1 to each bit will trigger SW_INT&lt;0-3> respectively to CM4."]
pub type PulseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Write_pulse bit field: writing 1'b1 to each bit will trigger SW_INT&lt;0-3> respectively to CM4."]
    #[inline(always)]
    pub fn pulse(&self) -> PulseR {
        PulseR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Write_pulse bit field: writing 1'b1 to each bit will trigger SW_INT&lt;0-3> respectively to CM4."]
    #[inline(always)]
    #[must_use]
    pub fn pulse(&mut self) -> PulseW<AppssSwIntSpec> {
        PulseW::new(self, 0)
    }
}
#[doc = "APPSS_SW_INT\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_sw_int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_sw_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssSwIntSpec;
impl crate::RegisterSpec for AppssSwIntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_sw_int::R`](R) reader structure"]
impl crate::Readable for AppssSwIntSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_sw_int::W`](W) writer structure"]
impl crate::Writable for AppssSwIntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_SW_INT to value 0"]
impl crate::Resettable for AppssSwIntSpec {
    const RESET_VALUE: u32 = 0;
}
