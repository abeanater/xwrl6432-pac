#[doc = "Register `CFARTEST` reader"]
pub type R = crate::R<CfartestSpec>;
#[doc = "Register `CFARTEST` writer"]
pub type W = crate::W<CfartestSpec>;
#[doc = "Field `CFARTEST` reader - 23:0\\]
Reserved.TI internal"]
pub type CfartestR = crate::FieldReader<u32>;
#[doc = "Field `CFARTEST` writer - 23:0\\]
Reserved.TI internal"]
pub type CfartestW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn cfartest(&self) -> CfartestR {
        CfartestR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn cfartest(&mut self) -> CfartestW<CfartestSpec> {
        CfartestW::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<CfartestSpec> {
        NuW::new(self, 24)
    }
}
#[doc = "CFARTEST\n\nYou can [`read`](crate::Reg::read) this register and get [`cfartest::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfartest::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfartestSpec;
impl crate::RegisterSpec for CfartestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfartest::R`](R) reader structure"]
impl crate::Readable for CfartestSpec {}
#[doc = "`write(|w| ..)` method takes [`cfartest::W`](W) writer structure"]
impl crate::Writable for CfartestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFARTEST to value 0"]
impl crate::Resettable for CfartestSpec {
    const RESET_VALUE: u32 = 0;
}
