#[doc = "Register `APPSS_CM4_HALT` reader"]
pub type R = crate::R<AppssCm4HaltSpec>;
#[doc = "Register `APPSS_CM4_HALT` writer"]
pub type W = crate::W<AppssCm4HaltSpec>;
#[doc = "Field `halt` reader - 2:0\\]
RESERVED"]
pub type HaltR = crate::FieldReader;
#[doc = "Field `halt` writer - 2:0\\]
RESERVED"]
pub type HaltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
RESERVED"]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HaltW<AppssCm4HaltSpec> {
        HaltW::new(self, 0)
    }
}
#[doc = "APPSS_CM4_HALT\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_cm4_halt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_cm4_halt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssCm4HaltSpec;
impl crate::RegisterSpec for AppssCm4HaltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_cm4_halt::R`](R) reader structure"]
impl crate::Readable for AppssCm4HaltSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_cm4_halt::W`](W) writer structure"]
impl crate::Writable for AppssCm4HaltSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_CM4_HALT to value 0"]
impl crate::Resettable for AppssCm4HaltSpec {
    const RESET_VALUE: u32 = 0;
}
