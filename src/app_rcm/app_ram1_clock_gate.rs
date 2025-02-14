#[doc = "Register `APP_RAM1_CLOCK_GATE` reader"]
pub type R = crate::R<AppRam1ClockGateSpec>;
#[doc = "Register `APP_RAM1_CLOCK_GATE` writer"]
pub type W = crate::W<AppRam1ClockGateSpec>;
#[doc = "Field `enable` reader - 2:0\\]
3'b000 : Ungate clock to APP RAM1 3'b111 : Gate Clock to APP_RAM1"]
pub type EnableR = crate::FieldReader;
#[doc = "Field `enable` writer - 2:0\\]
3'b000 : Ungate clock to APP RAM1 3'b111 : Gate Clock to APP_RAM1"]
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
3'b000 : Ungate clock to APP RAM1 3'b111 : Gate Clock to APP_RAM1"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
3'b000 : Ungate clock to APP RAM1 3'b111 : Gate Clock to APP_RAM1"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<AppRam1ClockGateSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "APP_RAM1_CLOCK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`app_ram1_clock_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_ram1_clock_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppRam1ClockGateSpec;
impl crate::RegisterSpec for AppRam1ClockGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_ram1_clock_gate::R`](R) reader structure"]
impl crate::Readable for AppRam1ClockGateSpec {}
#[doc = "`write(|w| ..)` method takes [`app_ram1_clock_gate::W`](W) writer structure"]
impl crate::Writable for AppRam1ClockGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_RAM1_CLOCK_GATE to value 0"]
impl crate::Resettable for AppRam1ClockGateSpec {
    const RESET_VALUE: u32 = 0;
}
