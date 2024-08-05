#[doc = "Register `ICPID2` reader"]
pub type R = crate::R<Icpid2Spec>;
#[doc = "Register `ICPID2` writer"]
pub type W = crate::W<Icpid2Spec>;
#[doc = "Field `TYPE` reader - 7:0\\]
Identifies the type of peripheral. This value should be 0x05 - (RW )"]
pub type TypeR = crate::FieldReader;
#[doc = "Field `TYPE` writer - 7:0\\]
Identifies the type of peripheral. This value should be 0x05 - (RW )"]
pub type TypeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU` reader - 31:8\\]
Reserved."]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:8\\]
Reserved."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Identifies the type of peripheral. This value should be 0x05 - (RW )"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Identifies the type of peripheral. This value should be 0x05 - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TypeW<Icpid2Spec> {
        TypeW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Icpid2Spec> {
        NuW::new(self, 8)
    }
}
#[doc = "I2C Peripheral ID register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`icpid2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpid2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icpid2Spec;
impl crate::RegisterSpec for Icpid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icpid2::R`](R) reader structure"]
impl crate::Readable for Icpid2Spec {}
#[doc = "`write(|w| ..)` method takes [`icpid2::W`](W) writer structure"]
impl crate::Writable for Icpid2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICPID2 to value 0"]
impl crate::Resettable for Icpid2Spec {
    const RESET_VALUE: u32 = 0;
}
