#[doc = "Register `HWACCREG2RD` reader"]
pub type R = crate::R<Hwaccreg2rdSpec>;
#[doc = "Register `HWACCREG2RD` writer"]
pub type W = crate::W<Hwaccreg2rdSpec>;
#[doc = "Field `HWACCREG2RD` reader - 31:0\\]
Reserved.TI internal"]
pub type Hwaccreg2rdR = crate::FieldReader<u32>;
#[doc = "Field `HWACCREG2RD` writer - 31:0\\]
Reserved.TI internal"]
pub type Hwaccreg2rdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn hwaccreg2rd(&self) -> Hwaccreg2rdR {
        Hwaccreg2rdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn hwaccreg2rd(&mut self) -> Hwaccreg2rdW<Hwaccreg2rdSpec> {
        Hwaccreg2rdW::new(self, 0)
    }
}
#[doc = "HWACCREG2RD\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg2rd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg2rd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwaccreg2rdSpec;
impl crate::RegisterSpec for Hwaccreg2rdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwaccreg2rd::R`](R) reader structure"]
impl crate::Readable for Hwaccreg2rdSpec {}
#[doc = "`write(|w| ..)` method takes [`hwaccreg2rd::W`](W) writer structure"]
impl crate::Writable for Hwaccreg2rdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWACCREG2RD to value 0"]
impl crate::Resettable for Hwaccreg2rdSpec {
    const RESET_VALUE: u32 = 0;
}
