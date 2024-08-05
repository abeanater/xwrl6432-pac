#[doc = "Register `DCCSTATUS2` reader"]
pub type R = crate::R<Dccstatus2Spec>;
#[doc = "Register `DCCSTATUS2` writer"]
pub type W = crate::W<Dccstatus2Spec>;
#[doc = "Field `COUNT0_FIFO_EMPTY` reader - 0:0\\]
Count0 FIFO Empty Indicates whether Count0 FIFO is Empty. 0: Count0 FIFO is not empty 1: Count0 FIFO is empty."]
pub type Count0FifoEmptyR = crate::BitReader;
#[doc = "Field `COUNT0_FIFO_EMPTY` writer - 0:0\\]
Count0 FIFO Empty Indicates whether Count0 FIFO is Empty. 0: Count0 FIFO is not empty 1: Count0 FIFO is empty."]
pub type Count0FifoEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VALID0_FIFO_EMPTY` reader - 1:1\\]
Valid0 FIFO Empty Indicates whether Valid0 FIFO is Empty. 0: Valid0 FIFO is not empty 1: Valid0 FIFO is empty."]
pub type Valid0FifoEmptyR = crate::BitReader;
#[doc = "Field `VALID0_FIFO_EMPTY` writer - 1:1\\]
Valid0 FIFO Empty Indicates whether Valid0 FIFO is Empty. 0: Valid0 FIFO is not empty 1: Valid0 FIFO is empty."]
pub type Valid0FifoEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNT1_FIFO_EMPTY` reader - 2:2\\]
Count1 FIFO Empty Indicates whether Count1 FIFO is Empty. 0: Count1 FIFO is not empty 1: Count1 FIFO is empty."]
pub type Count1FifoEmptyR = crate::BitReader;
#[doc = "Field `COUNT1_FIFO_EMPTY` writer - 2:2\\]
Count1 FIFO Empty Indicates whether Count1 FIFO is Empty. 0: Count1 FIFO is not empty 1: Count1 FIFO is empty."]
pub type Count1FifoEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNT0_FIFO_FULL` reader - 3:3\\]
Count0 FIFO Full Indicates whether Count0 FIFO is Full. 0: Count0 FIFO is not Full 1: Count0 FIFO is Full."]
pub type Count0FifoFullR = crate::BitReader;
#[doc = "Field `COUNT0_FIFO_FULL` writer - 3:3\\]
Count0 FIFO Full Indicates whether Count0 FIFO is Full. 0: Count0 FIFO is not Full 1: Count0 FIFO is Full."]
pub type Count0FifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VALID0_FIFO_FULL` reader - 4:4\\]
Valid0 FIFO Full Indicates whether Valid0 FIFO is Full. 0: Valid0 FIFO is not Full 1: Valid0 FIFO is Full."]
pub type Valid0FifoFullR = crate::BitReader;
#[doc = "Field `VALID0_FIFO_FULL` writer - 4:4\\]
Valid0 FIFO Full Indicates whether Valid0 FIFO is Full. 0: Valid0 FIFO is not Full 1: Valid0 FIFO is Full."]
pub type Valid0FifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNT1_FIFO_FULL` reader - 5:5\\]
Count1 FIFO Full Indicates whether Count1 FIFO is Full. 0: Count1 FIFO is not Full 1: Count1 FIFO is Full."]
pub type Count1FifoFullR = crate::BitReader;
#[doc = "Field `COUNT1_FIFO_FULL` writer - 5:5\\]
Count1 FIFO Full Indicates whether Count1 FIFO is Full. 0: Count1 FIFO is not Full 1: Count1 FIFO is Full."]
pub type Count1FifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU14` reader - 31:6\\]
Reserved"]
pub type Nu14R = crate::FieldReader<u32>;
#[doc = "Field `NU14` writer - 31:6\\]
Reserved"]
pub type Nu14W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Count0 FIFO Empty Indicates whether Count0 FIFO is Empty. 0: Count0 FIFO is not empty 1: Count0 FIFO is empty."]
    #[inline(always)]
    pub fn count0_fifo_empty(&self) -> Count0FifoEmptyR {
        Count0FifoEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Valid0 FIFO Empty Indicates whether Valid0 FIFO is Empty. 0: Valid0 FIFO is not empty 1: Valid0 FIFO is empty."]
    #[inline(always)]
    pub fn valid0_fifo_empty(&self) -> Valid0FifoEmptyR {
        Valid0FifoEmptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Count1 FIFO Empty Indicates whether Count1 FIFO is Empty. 0: Count1 FIFO is not empty 1: Count1 FIFO is empty."]
    #[inline(always)]
    pub fn count1_fifo_empty(&self) -> Count1FifoEmptyR {
        Count1FifoEmptyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Count0 FIFO Full Indicates whether Count0 FIFO is Full. 0: Count0 FIFO is not Full 1: Count0 FIFO is Full."]
    #[inline(always)]
    pub fn count0_fifo_full(&self) -> Count0FifoFullR {
        Count0FifoFullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Valid0 FIFO Full Indicates whether Valid0 FIFO is Full. 0: Valid0 FIFO is not Full 1: Valid0 FIFO is Full."]
    #[inline(always)]
    pub fn valid0_fifo_full(&self) -> Valid0FifoFullR {
        Valid0FifoFullR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Count1 FIFO Full Indicates whether Count1 FIFO is Full. 0: Count1 FIFO is not Full 1: Count1 FIFO is Full."]
    #[inline(always)]
    pub fn count1_fifo_full(&self) -> Count1FifoFullR {
        Count1FifoFullR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Reserved"]
    #[inline(always)]
    pub fn nu14(&self) -> Nu14R {
        Nu14R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Count0 FIFO Empty Indicates whether Count0 FIFO is Empty. 0: Count0 FIFO is not empty 1: Count0 FIFO is empty."]
    #[inline(always)]
    #[must_use]
    pub fn count0_fifo_empty(&mut self) -> Count0FifoEmptyW<Dccstatus2Spec> {
        Count0FifoEmptyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Valid0 FIFO Empty Indicates whether Valid0 FIFO is Empty. 0: Valid0 FIFO is not empty 1: Valid0 FIFO is empty."]
    #[inline(always)]
    #[must_use]
    pub fn valid0_fifo_empty(&mut self) -> Valid0FifoEmptyW<Dccstatus2Spec> {
        Valid0FifoEmptyW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Count1 FIFO Empty Indicates whether Count1 FIFO is Empty. 0: Count1 FIFO is not empty 1: Count1 FIFO is empty."]
    #[inline(always)]
    #[must_use]
    pub fn count1_fifo_empty(&mut self) -> Count1FifoEmptyW<Dccstatus2Spec> {
        Count1FifoEmptyW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Count0 FIFO Full Indicates whether Count0 FIFO is Full. 0: Count0 FIFO is not Full 1: Count0 FIFO is Full."]
    #[inline(always)]
    #[must_use]
    pub fn count0_fifo_full(&mut self) -> Count0FifoFullW<Dccstatus2Spec> {
        Count0FifoFullW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Valid0 FIFO Full Indicates whether Valid0 FIFO is Full. 0: Valid0 FIFO is not Full 1: Valid0 FIFO is Full."]
    #[inline(always)]
    #[must_use]
    pub fn valid0_fifo_full(&mut self) -> Valid0FifoFullW<Dccstatus2Spec> {
        Valid0FifoFullW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Count1 FIFO Full Indicates whether Count1 FIFO is Full. 0: Count1 FIFO is not Full 1: Count1 FIFO is Full."]
    #[inline(always)]
    #[must_use]
    pub fn count1_fifo_full(&mut self) -> Count1FifoFullW<Dccstatus2Spec> {
        Count1FifoFullW::new(self, 5)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu14(&mut self) -> Nu14W<Dccstatus2Spec> {
        Nu14W::new(self, 6)
    }
}
#[doc = "FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dccstatus2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccstatus2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dccstatus2Spec;
impl crate::RegisterSpec for Dccstatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dccstatus2::R`](R) reader structure"]
impl crate::Readable for Dccstatus2Spec {}
#[doc = "`write(|w| ..)` method takes [`dccstatus2::W`](W) writer structure"]
impl crate::Writable for Dccstatus2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCCSTATUS2 to value 0"]
impl crate::Resettable for Dccstatus2Spec {
    const RESET_VALUE: u32 = 0;
}
