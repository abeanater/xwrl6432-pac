#[doc = "Register `DCCSTAT` reader"]
pub type R = crate::R<DccstatSpec>;
#[doc = "Register `DCCSTAT` writer"]
pub type W = crate::W<DccstatSpec>;
#[doc = "Field `ERR` reader - 0:0\\]
Indicates whether or not an error has occured. Writing a 1 to this bit clears the flag."]
pub type ErrR = crate::BitReader;
#[doc = "Field `ERR` writer - 0:0\\]
Indicates whether or not an error has occured. Writing a 1 to this bit clears the flag."]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE` reader - 1:1\\]
Indicates whether or not an done has occured. Writing a 1 to this bit clears the flag."]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - 1:1\\]
Indicates whether or not an done has occured. Writing a 1 to this bit clears the flag."]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU6` reader - 31:2\\]
Reserved"]
pub type Nu6R = crate::FieldReader<u32>;
#[doc = "Field `NU6` writer - 31:2\\]
Reserved"]
pub type Nu6W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates whether or not an error has occured. Writing a 1 to this bit clears the flag."]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates whether or not an done has occured. Writing a 1 to this bit clears the flag."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    pub fn nu6(&self) -> Nu6R {
        Nu6R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates whether or not an error has occured. Writing a 1 to this bit clears the flag."]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ErrW<DccstatSpec> {
        ErrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates whether or not an done has occured. Writing a 1 to this bit clears the flag."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<DccstatSpec> {
        DoneW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu6(&mut self) -> Nu6W<DccstatSpec> {
        Nu6W::new(self, 2)
    }
}
#[doc = "Contains the error &amp; done flag bit\n\nYou can [`read`](crate::Reg::read) this register and get [`dccstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DccstatSpec;
impl crate::RegisterSpec for DccstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dccstat::R`](R) reader structure"]
impl crate::Readable for DccstatSpec {}
#[doc = "`write(|w| ..)` method takes [`dccstat::W`](W) writer structure"]
impl crate::Writable for DccstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCCSTAT to value 0"]
impl crate::Resettable for DccstatSpec {
    const RESET_VALUE: u32 = 0;
}
