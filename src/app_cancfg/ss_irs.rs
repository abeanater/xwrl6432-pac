#[doc = "Register `SS_IRS` reader"]
pub type R = crate::R<SsIrsSpec>;
#[doc = "Register `SS_IRS` writer"]
pub type W = crate::W<SsIrsSpec>;
#[doc = "Field `IRS` reader - 0:0\\]
External TimeStamp Counter Overflow Interrupt status. Read raw interrupt status. (IRS - Interrupt Raw Status Register)"]
pub type IrsR = crate::BitReader;
#[doc = "Field `IRS` writer - 0:0\\]
External TimeStamp Counter Overflow Interrupt status. Read raw interrupt status. (IRS - Interrupt Raw Status Register)"]
pub type IrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU3` reader - 31:1\\]
Reserved"]
pub type Nu3R = crate::FieldReader<u32>;
#[doc = "Field `NU3` writer - 31:1\\]
Reserved"]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
External TimeStamp Counter Overflow Interrupt status. Read raw interrupt status. (IRS - Interrupt Raw Status Register)"]
    #[inline(always)]
    pub fn irs(&self) -> IrsR {
        IrsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
External TimeStamp Counter Overflow Interrupt status. Read raw interrupt status. (IRS - Interrupt Raw Status Register)"]
    #[inline(always)]
    #[must_use]
    pub fn irs(&mut self) -> IrsW<SsIrsSpec> {
        IrsW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<SsIrsSpec> {
        Nu3W::new(self, 1)
    }
}
#[doc = "SS_IRS\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_irs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_irs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsIrsSpec;
impl crate::RegisterSpec for SsIrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_irs::R`](R) reader structure"]
impl crate::Readable for SsIrsSpec {}
#[doc = "`write(|w| ..)` method takes [`ss_irs::W`](W) writer structure"]
impl crate::Writable for SsIrsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SS_IRS to value 0"]
impl crate::Resettable for SsIrsSpec {
    const RESET_VALUE: u32 = 0;
}
