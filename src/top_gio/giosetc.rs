#[doc = "Register `GIOSETC` reader"]
pub type R = crate::R<GiosetcSpec>;
#[doc = "Register `GIOSETC` writer"]
pub type W = crate::W<GiosetcSpec>;
#[doc = "Field `GIODSETC` reader - 7:0\\]
GIO data set for port C"]
pub type GiodsetcR = crate::FieldReader;
#[doc = "Field `GIODSETC` writer - 7:0\\]
GIO data set for port C"]
pub type GiodsetcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU25` reader - 31:8\\]
Reserved"]
pub type Nu25R = crate::FieldReader<u32>;
#[doc = "Field `NU25` writer - 31:8\\]
Reserved"]
pub type Nu25W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data set for port C"]
    #[inline(always)]
    pub fn giodsetc(&self) -> GiodsetcR {
        GiodsetcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu25(&self) -> Nu25R {
        Nu25R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data set for port C"]
    #[inline(always)]
    #[must_use]
    pub fn giodsetc(&mut self) -> GiodsetcW<GiosetcSpec> {
        GiodsetcW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu25(&mut self) -> Nu25W<GiosetcSpec> {
        Nu25W::new(self, 8)
    }
}
#[doc = "GIO data set for port C\n\nYou can [`read`](crate::Reg::read) this register and get [`giosetc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosetc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiosetcSpec;
impl crate::RegisterSpec for GiosetcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giosetc::R`](R) reader structure"]
impl crate::Readable for GiosetcSpec {}
#[doc = "`write(|w| ..)` method takes [`giosetc::W`](W) writer structure"]
impl crate::Writable for GiosetcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOSETC to value 0"]
impl crate::Resettable for GiosetcSpec {
    const RESET_VALUE: u32 = 0;
}
