#[doc = "Register `APPSS_CTI_TRIG_SEL` reader"]
pub type R = crate::R<AppssCtiTrigSelSpec>;
#[doc = "Register `APPSS_CTI_TRIG_SEL` writer"]
pub type W = crate::W<AppssCtiTrigSelSpec>;
#[doc = "Field `trig8_sel` reader - 7:0\\]
Used for selecting the trigger source for 8th trigger of CTI"]
pub type Trig8SelR = crate::FieldReader;
#[doc = "Field `trig8_sel` writer - 7:0\\]
Used for selecting the trigger source for 8th trigger of CTI"]
pub type Trig8SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Used for selecting the trigger source for 8th trigger of CTI"]
    #[inline(always)]
    pub fn trig8_sel(&self) -> Trig8SelR {
        Trig8SelR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Used for selecting the trigger source for 8th trigger of CTI"]
    #[inline(always)]
    #[must_use]
    pub fn trig8_sel(&mut self) -> Trig8SelW<AppssCtiTrigSelSpec> {
        Trig8SelW::new(self, 0)
    }
}
#[doc = "APPSS_CTI_TRIG_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_cti_trig_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_cti_trig_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssCtiTrigSelSpec;
impl crate::RegisterSpec for AppssCtiTrigSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_cti_trig_sel::R`](R) reader structure"]
impl crate::Readable for AppssCtiTrigSelSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_cti_trig_sel::W`](W) writer structure"]
impl crate::Writable for AppssCtiTrigSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_CTI_TRIG_SEL to value 0"]
impl crate::Resettable for AppssCtiTrigSelSpec {
    const RESET_VALUE: u32 = 0;
}
