#[doc = "Register `XIDAM` reader"]
pub type R = crate::R<XidamSpec>;
#[doc = "Register `XIDAM` writer"]
pub type W = crate::W<XidamSpec>;
#[doc = "Field `EIDM` reader - 28:0\\]
Extended ID Mask"]
pub type EidmR = crate::FieldReader<u32>;
#[doc = "Field `EIDM` writer - 28:0\\]
Extended ID Mask"]
pub type EidmW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `NU39` reader - 31:29\\]
Reserved"]
pub type Nu39R = crate::FieldReader;
#[doc = "Field `NU39` writer - 31:29\\]
Reserved"]
pub type Nu39W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:28 - 28:0\\]
Extended ID Mask"]
    #[inline(always)]
    pub fn eidm(&self) -> EidmR {
        EidmR::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Reserved"]
    #[inline(always)]
    pub fn nu39(&self) -> Nu39R {
        Nu39R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:28 - 28:0\\]
Extended ID Mask"]
    #[inline(always)]
    #[must_use]
    pub fn eidm(&mut self) -> EidmW<XidamSpec> {
        EidmW::new(self, 0)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu39(&mut self) -> Nu39W<XidamSpec> {
        Nu39W::new(self, 29)
    }
}
#[doc = "XIDAM\n\nYou can [`read`](crate::Reg::read) this register and get [`xidam::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidam::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XidamSpec;
impl crate::RegisterSpec for XidamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xidam::R`](R) reader structure"]
impl crate::Readable for XidamSpec {}
#[doc = "`write(|w| ..)` method takes [`xidam::W`](W) writer structure"]
impl crate::Writable for XidamSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XIDAM to value 0"]
impl crate::Resettable for XidamSpec {
    const RESET_VALUE: u32 = 0;
}
