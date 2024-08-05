#[doc = "Register `DCCGCTRL2` reader"]
pub type R = crate::R<Dccgctrl2Spec>;
#[doc = "Register `DCCGCTRL2` writer"]
pub type W = crate::W<Dccgctrl2Spec>;
#[doc = "Field `CONT_ON_ERR` reader - 3:0\\]
Continue on Error enable Continues to next window of comparison despite the error condition. 0101: Comparison and counter reload is stopped from advancing if error is detected. Others: Counters get reloaded with seed and continue counting despite the error condition. Note: The user should write 1010 to these enable fields to enable each feature to avoid single soft errors."]
pub type ContOnErrR = crate::FieldReader;
#[doc = "Field `CONT_ON_ERR` writer - 3:0\\]
Continue on Error enable Continues to next window of comparison despite the error condition. 0101: Comparison and counter reload is stopped from advancing if error is detected. Others: Counters get reloaded with seed and continue counting despite the error condition. Note: The user should write 1010 to these enable fields to enable each feature to avoid single soft errors."]
pub type ContOnErrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FIFO_READ` reader - 7:4\\]
FIFO Read Enable Enables the counter read registers reflect FIFO output instead of live counter value. 0101: Counter value is read directly. Others: FIFO output is read Note: The user should write 1010 to these enable fields to enable each feature to avoid single soft errors."]
pub type FifoReadR = crate::FieldReader;
#[doc = "Field `FIFO_READ` writer - 7:4\\]
FIFO Read Enable Enables the counter read registers reflect FIFO output instead of live counter value. 0101: Counter value is read directly. Others: FIFO output is read Note: The user should write 1010 to these enable fields to enable each feature to avoid single soft errors."]
pub type FifoReadW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FIFO_NONERR` reader - 11:8\\]
FIFO update on Non-Error Enables/disables FIFO writes without the error event on completion of comparison window. 0101: Counter values are captured to non-full FIFO only upon Error event Others: Write counter values to non-full FIFO upon completion of comparison window regardless of error or not. Note this setting is applicable only in Continuous mode; in single shot mode FIFO captures counts only on Error. Note: The user should write 1010 to these enable fields to enable each feature to avoid single soft errors."]
pub type FifoNonerrR = crate::FieldReader;
#[doc = "Field `FIFO_NONERR` writer - 11:8\\]
FIFO update on Non-Error Enables/disables FIFO writes without the error event on completion of comparison window. 0101: Counter values are captured to non-full FIFO only upon Error event Others: Write counter values to non-full FIFO upon completion of comparison window regardless of error or not. Note this setting is applicable only in Continuous mode; in single shot mode FIFO captures counts only on Error. Note: The user should write 1010 to these enable fields to enable each feature to avoid single soft errors."]
pub type FifoNonerrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU13` reader - "]
pub type Nu13R = crate::FieldReader<u32>;
#[doc = "Field `NU13` writer - "]
pub type Nu13W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Continue on Error enable Continues to next window of comparison despite the error condition. 0101: Comparison and counter reload is stopped from advancing if error is detected. Others: Counters get reloaded with seed and continue counting despite the error condition. Note: The user should write 1010 to these enable fields to enable each feature to avoid single soft errors."]
    #[inline(always)]
    pub fn cont_on_err(&self) -> ContOnErrR {
        ContOnErrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
FIFO Read Enable Enables the counter read registers reflect FIFO output instead of live counter value. 0101: Counter value is read directly. Others: FIFO output is read Note: The user should write 1010 to these enable fields to enable each feature to avoid single soft errors."]
    #[inline(always)]
    pub fn fifo_read(&self) -> FifoReadR {
        FifoReadR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
FIFO update on Non-Error Enables/disables FIFO writes without the error event on completion of comparison window. 0101: Counter values are captured to non-full FIFO only upon Error event Others: Write counter values to non-full FIFO upon completion of comparison window regardless of error or not. Note this setting is applicable only in Continuous mode; in single shot mode FIFO captures counts only on Error. Note: The user should write 1010 to these enable fields to enable each feature to avoid single soft errors."]
    #[inline(always)]
    pub fn fifo_nonerr(&self) -> FifoNonerrR {
        FifoNonerrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:31"]
    #[inline(always)]
    pub fn nu13(&self) -> Nu13R {
        Nu13R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Continue on Error enable Continues to next window of comparison despite the error condition. 0101: Comparison and counter reload is stopped from advancing if error is detected. Others: Counters get reloaded with seed and continue counting despite the error condition. Note: The user should write 1010 to these enable fields to enable each feature to avoid single soft errors."]
    #[inline(always)]
    #[must_use]
    pub fn cont_on_err(&mut self) -> ContOnErrW<Dccgctrl2Spec> {
        ContOnErrW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
FIFO Read Enable Enables the counter read registers reflect FIFO output instead of live counter value. 0101: Counter value is read directly. Others: FIFO output is read Note: The user should write 1010 to these enable fields to enable each feature to avoid single soft errors."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_read(&mut self) -> FifoReadW<Dccgctrl2Spec> {
        FifoReadW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
FIFO update on Non-Error Enables/disables FIFO writes without the error event on completion of comparison window. 0101: Counter values are captured to non-full FIFO only upon Error event Others: Write counter values to non-full FIFO upon completion of comparison window regardless of error or not. Note this setting is applicable only in Continuous mode; in single shot mode FIFO captures counts only on Error. Note: The user should write 1010 to these enable fields to enable each feature to avoid single soft errors."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_nonerr(&mut self) -> FifoNonerrW<Dccgctrl2Spec> {
        FifoNonerrW::new(self, 8)
    }
    #[doc = "Bits 12:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu13(&mut self) -> Nu13W<Dccgctrl2Spec> {
        Nu13W::new(self, 12)
    }
}
#[doc = "Global control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dccgctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccgctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dccgctrl2Spec;
impl crate::RegisterSpec for Dccgctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dccgctrl2::R`](R) reader structure"]
impl crate::Readable for Dccgctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`dccgctrl2::W`](W) writer structure"]
impl crate::Writable for Dccgctrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCCGCTRL2 to value 0"]
impl crate::Resettable for Dccgctrl2Spec {
    const RESET_VALUE: u32 = 0;
}
