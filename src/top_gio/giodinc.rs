#[doc = "Register `GIODINC` reader"]
pub type R = crate::R<GiodincSpec>;
#[doc = "Register `GIODINC` writer"]
pub type W = crate::W<GiodincSpec>;
#[doc = "Field `GIODINC` reader - 7:0\\]
GIO data input for pins in port C"]
pub type GiodincR = crate::FieldReader;
#[doc = "Field `GIODINC` writer - 7:0\\]
GIO data input for pins in port C"]
pub type GiodincW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU13` reader - 31:8\\]
Reserved"]
pub type Nu13R = crate::FieldReader<u32>;
#[doc = "Field `NU13` writer - 31:8\\]
Reserved"]
pub type Nu13W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data input for pins in port C"]
    #[inline(always)]
    pub fn giodinc(&self) -> GiodincR {
        GiodincR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu13(&self) -> Nu13R {
        Nu13R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data input for pins in port C"]
    #[inline(always)]
    #[must_use]
    pub fn giodinc(&mut self) -> GiodincW<GiodincSpec> {
        GiodincW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu13(&mut self) -> Nu13W<GiodincSpec> {
        Nu13W::new(self, 8)
    }
}
#[doc = "GIO data input for pins in port C\n\nYou can [`read`](crate::Reg::read) this register and get [`giodinc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodinc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodincSpec;
impl crate::RegisterSpec for GiodincSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodinc::R`](R) reader structure"]
impl crate::Readable for GiodincSpec {}
#[doc = "`write(|w| ..)` method takes [`giodinc::W`](W) writer structure"]
impl crate::Writable for GiodincSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODINC to value 0"]
impl crate::Resettable for GiodincSpec {
    const RESET_VALUE: u32 = 0;
}
