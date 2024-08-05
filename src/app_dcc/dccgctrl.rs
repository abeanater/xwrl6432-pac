#[doc = "Register `DCCGCTRL` reader"]
pub type R = crate::R<DccgctrlSpec>;
#[doc = "Register `DCCGCTRL` writer"]
pub type W = crate::W<DccgctrlSpec>;
#[doc = "Field `DCCENA` reader - 3:0\\]
The DCCENA bit starts and stops the operation of the dcc 0101 = disabled &amp; 1010 = enabled"]
pub type DccenaR = crate::FieldReader;
#[doc = "Field `DCCENA` writer - 3:0\\]
The DCCENA bit starts and stops the operation of the dcc 0101 = disabled &amp; 1010 = enabled"]
pub type DccenaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ERRENA` reader - 7:4\\]
The ERRENA bit enables/disables the error signal. 0101 = disabled &amp; 1010 = enabled"]
pub type ErrenaR = crate::FieldReader;
#[doc = "Field `ERRENA` writer - 7:4\\]
The ERRENA bit enables/disables the error signal. 0101 = disabled &amp; 1010 = enabled"]
pub type ErrenaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SINGLESHOT` reader - 11:8\\]
Single/Continuous checking mode. 0101 = Continuous &amp; 1010 = Single"]
pub type SingleshotR = crate::FieldReader;
#[doc = "Field `SINGLESHOT` writer - 11:8\\]
Single/Continuous checking mode. 0101 = Continuous &amp; 1010 = Single"]
pub type SingleshotW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DONENA` reader - 15:12\\]
The DONEENA bit enables/disables the done signal. 0101 = disabled &amp; 1010 = enabled"]
pub type DonenaR = crate::FieldReader;
#[doc = "Field `DONENA` writer - 15:12\\]
The DONEENA bit enables/disables the done signal. 0101 = disabled &amp; 1010 = enabled"]
pub type DonenaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU` reader - 31:16\\]
Reserved"]
pub type NuR = crate::FieldReader<u16>;
#[doc = "Field `NU` writer - 31:16\\]
Reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
The DCCENA bit starts and stops the operation of the dcc 0101 = disabled &amp; 1010 = enabled"]
    #[inline(always)]
    pub fn dccena(&self) -> DccenaR {
        DccenaR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
The ERRENA bit enables/disables the error signal. 0101 = disabled &amp; 1010 = enabled"]
    #[inline(always)]
    pub fn errena(&self) -> ErrenaR {
        ErrenaR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Single/Continuous checking mode. 0101 = Continuous &amp; 1010 = Single"]
    #[inline(always)]
    pub fn singleshot(&self) -> SingleshotR {
        SingleshotR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
The DONEENA bit enables/disables the done signal. 0101 = disabled &amp; 1010 = enabled"]
    #[inline(always)]
    pub fn donena(&self) -> DonenaR {
        DonenaR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
The DCCENA bit starts and stops the operation of the dcc 0101 = disabled &amp; 1010 = enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dccena(&mut self) -> DccenaW<DccgctrlSpec> {
        DccenaW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
The ERRENA bit enables/disables the error signal. 0101 = disabled &amp; 1010 = enabled"]
    #[inline(always)]
    #[must_use]
    pub fn errena(&mut self) -> ErrenaW<DccgctrlSpec> {
        ErrenaW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Single/Continuous checking mode. 0101 = Continuous &amp; 1010 = Single"]
    #[inline(always)]
    #[must_use]
    pub fn singleshot(&mut self) -> SingleshotW<DccgctrlSpec> {
        SingleshotW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
The DONEENA bit enables/disables the done signal. 0101 = disabled &amp; 1010 = enabled"]
    #[inline(always)]
    #[must_use]
    pub fn donena(&mut self) -> DonenaW<DccgctrlSpec> {
        DonenaW::new(self, 12)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<DccgctrlSpec> {
        NuW::new(self, 16)
    }
}
#[doc = "Starts / stops the counters clears the error signal\n\nYou can [`read`](crate::Reg::read) this register and get [`dccgctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccgctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DccgctrlSpec;
impl crate::RegisterSpec for DccgctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dccgctrl::R`](R) reader structure"]
impl crate::Readable for DccgctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dccgctrl::W`](W) writer structure"]
impl crate::Writable for DccgctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCCGCTRL to value 0"]
impl crate::Resettable for DccgctrlSpec {
    const RESET_VALUE: u32 = 0;
}
