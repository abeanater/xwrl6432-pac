#[doc = "Register `HWACCREG3RD` reader"]
pub type R = crate::R<Hwaccreg3rdSpec>;
#[doc = "Register `HWACCREG3RD` writer"]
pub type W = crate::W<Hwaccreg3rdSpec>;
#[doc = "Field `HWACCREG3RD` reader - 31:0\\]
Reserved.TI internal"]
pub type Hwaccreg3rdR = crate::FieldReader<u32>;
#[doc = "Field `HWACCREG3RD` writer - 31:0\\]
Reserved.TI internal"]
pub type Hwaccreg3rdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn hwaccreg3rd(&self) -> Hwaccreg3rdR {
        Hwaccreg3rdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn hwaccreg3rd(&mut self) -> Hwaccreg3rdW<Hwaccreg3rdSpec> {
        Hwaccreg3rdW::new(self, 0)
    }
}
#[doc = "HWACCREG3RD\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg3rd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg3rd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwaccreg3rdSpec;
impl crate::RegisterSpec for Hwaccreg3rdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwaccreg3rd::R`](R) reader structure"]
impl crate::Readable for Hwaccreg3rdSpec {}
#[doc = "`write(|w| ..)` method takes [`hwaccreg3rd::W`](W) writer structure"]
impl crate::Writable for Hwaccreg3rdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWACCREG3RD to value 0"]
impl crate::Resettable for Hwaccreg3rdSpec {
    const RESET_VALUE: u32 = 0;
}
