#[doc = "Register `APPSS_CM4_EVENT` reader"]
pub type R = crate::R<AppssCm4EventSpec>;
#[doc = "Register `APPSS_CM4_EVENT` writer"]
pub type W = crate::W<AppssCm4EventSpec>;
#[doc = "Field `cpu0_event` reader - 2:0\\]
Reserved Register for R &amp; D"]
pub type Cpu0EventR = crate::FieldReader;
#[doc = "Field `cpu0_event` writer - 2:0\\]
Reserved Register for R &amp; D"]
pub type Cpu0EventW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Reserved Register for R &amp; D"]
    #[inline(always)]
    pub fn cpu0_event(&self) -> Cpu0EventR {
        Cpu0EventR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Reserved Register for R &amp; D"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_event(&mut self) -> Cpu0EventW<AppssCm4EventSpec> {
        Cpu0EventW::new(self, 0)
    }
}
#[doc = "APPSS_CM4_EVENT\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_cm4_event::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_cm4_event::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssCm4EventSpec;
impl crate::RegisterSpec for AppssCm4EventSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_cm4_event::R`](R) reader structure"]
impl crate::Readable for AppssCm4EventSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_cm4_event::W`](W) writer structure"]
impl crate::Writable for AppssCm4EventSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_CM4_EVENT to value 0"]
impl crate::Resettable for AppssCm4EventSpec {
    const RESET_VALUE: u32 = 0;
}
