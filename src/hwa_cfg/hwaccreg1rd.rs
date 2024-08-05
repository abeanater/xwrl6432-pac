#[doc = "Register `HWACCREG1RD` reader"]
pub type R = crate::R<Hwaccreg1rdSpec>;
#[doc = "Register `HWACCREG1RD` writer"]
pub type W = crate::W<Hwaccreg1rdSpec>;
#[doc = "Field `HWACCREG1RD` reader - 31:0\\]
Reserved.TI internal"]
pub type Hwaccreg1rdR = crate::FieldReader<u32>;
#[doc = "Field `HWACCREG1RD` writer - 31:0\\]
Reserved.TI internal"]
pub type Hwaccreg1rdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn hwaccreg1rd(&self) -> Hwaccreg1rdR {
        Hwaccreg1rdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn hwaccreg1rd(&mut self) -> Hwaccreg1rdW<Hwaccreg1rdSpec> {
        Hwaccreg1rdW::new(self, 0)
    }
}
#[doc = "HWACCREG1RD\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg1rd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg1rd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwaccreg1rdSpec;
impl crate::RegisterSpec for Hwaccreg1rdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwaccreg1rd::R`](R) reader structure"]
impl crate::Readable for Hwaccreg1rdSpec {}
#[doc = "`write(|w| ..)` method takes [`hwaccreg1rd::W`](W) writer structure"]
impl crate::Writable for Hwaccreg1rdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWACCREG1RD to value 0"]
impl crate::Resettable for Hwaccreg1rdSpec {
    const RESET_VALUE: u32 = 0;
}
