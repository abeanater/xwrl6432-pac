#[doc = "Register `ECR` reader"]
pub type R = crate::R<EcrSpec>;
#[doc = "Register `ECR` writer"]
pub type W = crate::W<EcrSpec>;
#[doc = "Field `TEC` reader - 7:0\\]
Transmit Error Counter"]
pub type TecR = crate::FieldReader;
#[doc = "Field `TEC` writer - 7:0\\]
Transmit Error Counter"]
pub type TecW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REC` reader - 14:8\\]
Recieve Error Counter"]
pub type RecR = crate::FieldReader;
#[doc = "Field `REC` writer - 14:8\\]
Recieve Error Counter"]
pub type RecW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RP` reader - 15:15\\]
Recieve Error Passive"]
pub type RpR = crate::BitReader;
#[doc = "Field `RP` writer - 15:15\\]
Recieve Error Passive"]
pub type RpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEL` reader - 23:16\\]
CAN Error Logging"]
pub type CelR = crate::FieldReader;
#[doc = "Field `CEL` writer - 23:16\\]
CAN Error Logging"]
pub type CelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU25` reader - 31:24\\]
Reserved"]
pub type Nu25R = crate::FieldReader;
#[doc = "Field `NU25` writer - 31:24\\]
Reserved"]
pub type Nu25W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&self) -> TecR {
        TecR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Recieve Error Counter"]
    #[inline(always)]
    pub fn rec(&self) -> RecR {
        RecR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Recieve Error Passive"]
    #[inline(always)]
    pub fn rp(&self) -> RpR {
        RpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
CAN Error Logging"]
    #[inline(always)]
    pub fn cel(&self) -> CelR {
        CelR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    pub fn nu25(&self) -> Nu25R {
        Nu25R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Transmit Error Counter"]
    #[inline(always)]
    #[must_use]
    pub fn tec(&mut self) -> TecW<EcrSpec> {
        TecW::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Recieve Error Counter"]
    #[inline(always)]
    #[must_use]
    pub fn rec(&mut self) -> RecW<EcrSpec> {
        RecW::new(self, 8)
    }
    #[doc = "Bit 15 - 15:15\\]
Recieve Error Passive"]
    #[inline(always)]
    #[must_use]
    pub fn rp(&mut self) -> RpW<EcrSpec> {
        RpW::new(self, 15)
    }
    #[doc = "Bits 16:23 - 23:16\\]
CAN Error Logging"]
    #[inline(always)]
    #[must_use]
    pub fn cel(&mut self) -> CelW<EcrSpec> {
        CelW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu25(&mut self) -> Nu25W<EcrSpec> {
        Nu25W::new(self, 24)
    }
}
#[doc = "ECR\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcrSpec;
impl crate::RegisterSpec for EcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecr::R`](R) reader structure"]
impl crate::Readable for EcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ecr::W`](W) writer structure"]
impl crate::Writable for EcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for EcrSpec {
    const RESET_VALUE: u32 = 0;
}
