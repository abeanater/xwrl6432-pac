#[doc = "Register `ICDXR` reader"]
pub type R = crate::R<IcdxrSpec>;
#[doc = "Register `ICDXR` writer"]
pub type W = crate::W<IcdxrSpec>;
#[doc = "Field `D7_D0` reader - 7:0\\]
Transmit data"]
pub type D7D0R = crate::FieldReader;
#[doc = "Field `D7_D0` writer - 7:0\\]
Transmit data"]
pub type D7D0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU` reader - 31:8\\]
Reserved"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:8\\]
Reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Transmit data"]
    #[inline(always)]
    pub fn d7_d0(&self) -> D7D0R {
        D7D0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Transmit data"]
    #[inline(always)]
    #[must_use]
    pub fn d7_d0(&mut self) -> D7D0W<IcdxrSpec> {
        D7D0W::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<IcdxrSpec> {
        NuW::new(self, 8)
    }
}
#[doc = "I2C Data Transmit register\n\nYou can [`read`](crate::Reg::read) this register and get [`icdxr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icdxr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcdxrSpec;
impl crate::RegisterSpec for IcdxrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icdxr::R`](R) reader structure"]
impl crate::Readable for IcdxrSpec {}
#[doc = "`write(|w| ..)` method takes [`icdxr::W`](W) writer structure"]
impl crate::Writable for IcdxrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICDXR to value 0"]
impl crate::Resettable for IcdxrSpec {
    const RESET_VALUE: u32 = 0;
}
