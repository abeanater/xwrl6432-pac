#[doc = "Register `ICDRR` reader"]
pub type R = crate::R<IcdrrSpec>;
#[doc = "Register `ICDRR` writer"]
pub type W = crate::W<IcdrrSpec>;
#[doc = "Field `D7_D0` reader - 7:0\\]
Receive data"]
pub type D7D0R = crate::FieldReader;
#[doc = "Field `D7_D0` writer - 7:0\\]
Receive data"]
pub type D7D0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU` reader - 31:8\\]
Reserved"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:8\\]
Reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Receive data"]
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
Receive data"]
    #[inline(always)]
    #[must_use]
    pub fn d7_d0(&mut self) -> D7D0W<IcdrrSpec> {
        D7D0W::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<IcdrrSpec> {
        NuW::new(self, 8)
    }
}
#[doc = "I2C Data Receive register\n\nYou can [`read`](crate::Reg::read) this register and get [`icdrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icdrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcdrrSpec;
impl crate::RegisterSpec for IcdrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icdrr::R`](R) reader structure"]
impl crate::Readable for IcdrrSpec {}
#[doc = "`write(|w| ..)` method takes [`icdrr::W`](W) writer structure"]
impl crate::Writable for IcdrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICDRR to value 0"]
impl crate::Resettable for IcdrrSpec {
    const RESET_VALUE: u32 = 0;
}
