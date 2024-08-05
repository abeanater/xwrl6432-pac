#[doc = "Register `GIODINH` reader"]
pub type R = crate::R<GiodinhSpec>;
#[doc = "Register `GIODINH` writer"]
pub type W = crate::W<GiodinhSpec>;
#[doc = "Field `GIODINH` reader - 7:0\\]
GIO data input for pins in port H"]
pub type GiodinhR = crate::FieldReader;
#[doc = "Field `GIODINH` writer - 7:0\\]
GIO data input for pins in port H"]
pub type GiodinhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU16` reader - 31:8\\]
Reserved"]
pub type Nu16R = crate::FieldReader<u32>;
#[doc = "Field `NU16` writer - 31:8\\]
Reserved"]
pub type Nu16W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data input for pins in port H"]
    #[inline(always)]
    pub fn giodinh(&self) -> GiodinhR {
        GiodinhR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu16(&self) -> Nu16R {
        Nu16R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data input for pins in port H"]
    #[inline(always)]
    #[must_use]
    pub fn giodinh(&mut self) -> GiodinhW<GiodinhSpec> {
        GiodinhW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu16(&mut self) -> Nu16W<GiodinhSpec> {
        Nu16W::new(self, 8)
    }
}
#[doc = "GIO data input for pins in Port H\n\nYou can [`read`](crate::Reg::read) this register and get [`giodinh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodinh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodinhSpec;
impl crate::RegisterSpec for GiodinhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodinh::R`](R) reader structure"]
impl crate::Readable for GiodinhSpec {}
#[doc = "`write(|w| ..)` method takes [`giodinh::W`](W) writer structure"]
impl crate::Writable for GiodinhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODINH to value 0"]
impl crate::Resettable for GiodinhSpec {
    const RESET_VALUE: u32 = 0;
}
