#[doc = "Register `APP_ROM_CLOCK_GATE` reader"]
pub type R = crate::R<AppRomClockGateSpec>;
#[doc = "Register `APP_ROM_CLOCK_GATE` writer"]
pub type W = crate::W<AppRomClockGateSpec>;
#[doc = "Field `enable` reader - 2:0\\]
3'b000 : Ungate clock to APP ROM 3'b111 : Gate Clock to APP ROM"]
pub type EnableR = crate::FieldReader;
#[doc = "Field `enable` writer - 2:0\\]
3'b000 : Ungate clock to APP ROM 3'b111 : Gate Clock to APP ROM"]
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
3'b000 : Ungate clock to APP ROM 3'b111 : Gate Clock to APP ROM"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
3'b000 : Ungate clock to APP ROM 3'b111 : Gate Clock to APP ROM"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<AppRomClockGateSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "APP_ROM_CLOCK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`app_rom_clock_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_rom_clock_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppRomClockGateSpec;
impl crate::RegisterSpec for AppRomClockGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_rom_clock_gate::R`](R) reader structure"]
impl crate::Readable for AppRomClockGateSpec {}
#[doc = "`write(|w| ..)` method takes [`app_rom_clock_gate::W`](W) writer structure"]
impl crate::Writable for AppRomClockGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_ROM_CLOCK_GATE to value 0"]
impl crate::Resettable for AppRomClockGateSpec {
    const RESET_VALUE: u32 = 0;
}
