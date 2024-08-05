#[doc = "Register `GIODINF` reader"]
pub type R = crate::R<GiodinfSpec>;
#[doc = "Register `GIODINF` writer"]
pub type W = crate::W<GiodinfSpec>;
#[doc = "Field `GIODINF` reader - 7:0\\]
GIO data input for pins in port F"]
pub type GiodinfR = crate::FieldReader;
#[doc = "Field `GIODINF` writer - 7:0\\]
GIO data input for pins in port F"]
pub type GiodinfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU16` reader - 31:8\\]
Reserved"]
pub type Nu16R = crate::FieldReader<u32>;
#[doc = "Field `NU16` writer - 31:8\\]
Reserved"]
pub type Nu16W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data input for pins in port F"]
    #[inline(always)]
    pub fn giodinf(&self) -> GiodinfR {
        GiodinfR::new((self.bits & 0xff) as u8)
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
GIO data input for pins in port F"]
    #[inline(always)]
    #[must_use]
    pub fn giodinf(&mut self) -> GiodinfW<GiodinfSpec> {
        GiodinfW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu16(&mut self) -> Nu16W<GiodinfSpec> {
        Nu16W::new(self, 8)
    }
}
#[doc = "GIO data input for pins in Port F\n\nYou can [`read`](crate::Reg::read) this register and get [`giodinf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodinf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodinfSpec;
impl crate::RegisterSpec for GiodinfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodinf::R`](R) reader structure"]
impl crate::Readable for GiodinfSpec {}
#[doc = "`write(|w| ..)` method takes [`giodinf::W`](W) writer structure"]
impl crate::Writable for GiodinfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODINF to value 0"]
impl crate::Resettable for GiodinfSpec {
    const RESET_VALUE: u32 = 0;
}
