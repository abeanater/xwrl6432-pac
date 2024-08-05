#[doc = "Register `GIOPSLG` reader"]
pub type R = crate::R<GiopslgSpec>;
#[doc = "Register `GIOPSLG` writer"]
pub type W = crate::W<GiopslgSpec>;
#[doc = "Field `GIOPSLG` reader - 7:0\\]
GIO pull select for port G"]
pub type GiopslgR = crate::FieldReader;
#[doc = "Field `GIOPSLG` writer - 7:0\\]
GIO pull select for port G"]
pub type GiopslgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU39` reader - 31:8\\]
Reserved"]
pub type Nu39R = crate::FieldReader<u32>;
#[doc = "Field `NU39` writer - 31:8\\]
Reserved"]
pub type Nu39W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull select for port G"]
    #[inline(always)]
    pub fn giopslg(&self) -> GiopslgR {
        GiopslgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu39(&self) -> Nu39R {
        Nu39R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull select for port G"]
    #[inline(always)]
    #[must_use]
    pub fn giopslg(&mut self) -> GiopslgW<GiopslgSpec> {
        GiopslgW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu39(&mut self) -> Nu39W<GiopslgSpec> {
        Nu39W::new(self, 8)
    }
}
#[doc = "GIO pul select for port G\n\nYou can [`read`](crate::Reg::read) this register and get [`giopslg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopslg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiopslgSpec;
impl crate::RegisterSpec for GiopslgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giopslg::R`](R) reader structure"]
impl crate::Readable for GiopslgSpec {}
#[doc = "`write(|w| ..)` method takes [`giopslg::W`](W) writer structure"]
impl crate::Writable for GiopslgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOPSLG to value 0"]
impl crate::Resettable for GiopslgSpec {
    const RESET_VALUE: u32 = 0;
}
